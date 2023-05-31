use crate::errors::ContractError;
use crate::msgs::execute_msg::ExecuteMsg;
use crate::msgs::instantiate_msg::InstantiateMsg;
use crate::state::{
    Config, CONFIG, CRONCAT_FACTORY_ADDRESS, FUNDS_NECESSARY, PUBLIC_FUNDING_ADDRESS,
};
use crate::{CONTRACT_NAME, CONTRACT_VERSION};
use cosmwasm_std::CosmosMsg::Wasm;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{coin, entry_point, to_binary, Addr, Uint128};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use croncat_integration_utils::task_creation::create_croncat_task_message;
use croncat_integration_utils::{
    CronCatAction, CronCatBoundary, CronCatBoundaryTime, CronCatInterval, CronCatTaskRequest,
};
use cw2::set_contract_version;
use std::ops::Mul;

/// Instantiate entry point
/// See the instantiate message and fields in [InstantiateMsg](InstantiateMsg)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set the contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Save some initial config, shall we…
    let first_coin_sent = info.funds.first().ok_or(ContractError::NoFunds)?.clone();
    let initial_config = Config {
        number_of_tries: 1,
        denom: first_coin_sent.denom,
        max_times: msg.max_times_to_try,
        delay_in_minutes: msg.delay_in_minutes,
    };
    CONFIG.save(deps.storage, &initial_config)?;

    // Validate CronCat factory address
    let croncat_factory_address = deps
        .api
        .addr_validate(msg.croncat_factory_address.as_str())?;
    // Validate public funding address
    deps.api
        .addr_validate(msg.public_funding_address.as_str())?;

    // Save to state
    CRONCAT_FACTORY_ADDRESS.save(deps.storage, &croncat_factory_address)?;
    PUBLIC_FUNDING_ADDRESS.save(deps.storage, &Addr::unchecked(msg.public_funding_address))?;
    let first_fund = info.funds[0].clone();
    let starting_funds = first_fund.amount;
    let starting_denom = first_fund.denom;
    let double_the_starting_funds = starting_funds.mul(Uint128::from(2u128));
    FUNDS_NECESSARY.save(deps.storage, &double_the_starting_funds)?;

    // We want to call ourselves in 6 blocks from now. Hardcoded af.
    // let target_future_height = Uint64::from(env.block.height + 6);
    let target_future_time = env
        .block
        .time
        .plus_seconds(60 * msg.delay_in_minutes as u64);

    // Let's divide whatever the funds provided were by 4, why not
    let quarter_funds = starting_funds.checked_div(Uint128::from(4u8)).unwrap();

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
                contract_addr: env.contract.address.to_string(),
                msg: to_binary(&ExecuteMsg::CheckFundsDoThing {})?,
                funds: vec![coin(quarter_funds.u128(), starting_denom)],
            }),
            gas_limit: Some(300_000), // Can fine tune gas here
        }],
        queries: None,
        transforms: None,
        cw20: None,
    };

    let message =
        create_croncat_task_message(&deps.querier, info, croncat_factory_address, croncat_task);

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_message(message.unwrap()))
}
