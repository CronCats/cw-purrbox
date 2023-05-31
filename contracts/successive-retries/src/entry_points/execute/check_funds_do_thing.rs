//! Execute logic that explicitly sets the state boolean to `true` or `false`

use crate::errors::ContractError;
use crate::msgs::execute_msg::ExecuteMsg;
use crate::state::{
    Config, CONFIG, CRONCAT_FACTORY_ADDRESS, FUNDS_NECESSARY, PUBLIC_FUNDING_ADDRESS,
};
use cosmwasm_std::BankMsg::Send;
use cosmwasm_std::CosmosMsg::{Bank, Wasm};
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{to_binary, Coin, DepsMut, Env, MessageInfo, Response};
use croncat_integration_utils::task_creation::create_croncat_task_message;
use croncat_integration_utils::{
    CronCatAction, CronCatBoundary, CronCatBoundaryTime, CronCatInterval, CronCatTaskRequest,
};

/// Logic for the [CheckFundsDoThing](ExecuteMsg::CheckFundsDoThing) (`check_funds_do_thing`) method
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // Before all else, check if the retries are exhausted.
    // If they are, this contract is considered complete.
    let config = CONFIG.load(deps.storage)?;
    let current_retries = config.number_of_tries;
    let max_times = config.max_times;
    if current_retries >= max_times {
        return Err(ContractError::Completed);
    }

    let saved_denom = config.denom.clone();
    let delay_in_minutes = config.delay_in_minutes;
    let contract_address = &env.contract.address;

    // Increase retry count and save to state
    CONFIG.save(
        deps.storage,
        &Config {
            number_of_tries: current_retries + 1,
            denom: saved_denom.clone(),
            max_times,
            delay_in_minutes,
        },
    )?;

    // Load CronCat factory address from state
    let croncat_factory_addr = CRONCAT_FACTORY_ADDRESS.load(deps.storage)?;

    // Use CronCat helper to validate this invocation is coming from
    // a sanctioned version of a CronCat manager contract.
    let _task_info = croncat_integration_utils::handle_incoming_task::handle_incoming_task(
        &deps.querier,
        env.clone(),
        info.clone(),
        // Remember we load this from our contract's state.
        croncat_factory_addr.clone(),
        // We could use extra_params here, but this also means default.
        None,
    )?;

    // We've validated that this invocation is from a known CronCat task.
    // All code beyond this point can be comfortable in that knowledge.

    // Check to see if this contract has double the funds.
    let target_goal = FUNDS_NECESSARY.load(deps.storage)?;
    let current_contract_balance = deps
        .querier
        .query_balance(contract_address.clone(), config.denom)?
        .amount;

    if current_contract_balance < target_goal {
        // We're below the target goal still
        // We already increased the retry count earlier, so
        // create another task with backoff

        let target_future_time = env.block.time.plus_seconds(60 * delay_in_minutes as u64);

        // Create a task that fires every block, stops and refunds remaining
        // funds if the call fails.
        // This will have one Action: to call myself at the "check_funds_to_thing" method.
        let croncat_task = CronCatTaskRequest {
            interval: CronCatInterval::Once,
            boundary: Some(CronCatBoundary::Time(CronCatBoundaryTime {
                start: Some(target_future_time),
                end: None,
            })),
            stop_on_fail: true,
            actions: vec![CronCatAction {
                msg: Wasm(Execute {
                    contract_addr: contract_address.to_string(),
                    msg: to_binary(&ExecuteMsg::CheckFundsDoThing {})?,
                    funds: vec![],
                }),
                gas_limit: Some(300_000), // Can fine tune gas here
            }],
            queries: None,
            transforms: None,
            cw20: None,
        };

        let croncat_message =
            create_croncat_task_message(&deps.querier, info, croncat_factory_addr, croncat_task);
        Ok(Response::new().add_message(croncat_message.unwrap()))
    } else {
        // Hooray! They have reached their goal!
        // Get the address of the public funding DAO
        let public_funding_dao_address = PUBLIC_FUNDING_ADDRESS.load(deps.storage)?;

        // Send it all to them.
        let send_to_public_goods_message = Bank(Send {
            to_address: public_funding_dao_address.to_string(),
            amount: vec![Coin {
                denom: saved_denom.clone(),
                amount: current_contract_balance,
            }],
        });

        // Before ending, mark as complete by setting max to zero
        // This example is so dumb, I know
        CONFIG.save(
            deps.storage,
            &Config {
                number_of_tries: config.number_of_tries,
                denom: saved_denom,
                max_times: 0,
                delay_in_minutes: config.delay_in_minutes,
            },
        )?;

        Ok(Response::new().add_message(send_to_public_goods_message))
    }
}
