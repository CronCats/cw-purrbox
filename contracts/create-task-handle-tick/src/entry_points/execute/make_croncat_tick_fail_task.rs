use crate::errors::ContractError;
use crate::msgs::execute_msg::ExecuteMsg;
use crate::state::CRONCAT_FACTORY_ADDRESS;
use cosmwasm_std::CosmosMsg::Wasm;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response};
use croncat_integration_utils::task_creation::create_croncat_task_submessage;
use croncat_integration_utils::{CronCatAction, CronCatInterval, CronCatTaskRequest};

/// This method intentionally fails because it doesn't
/// attach funds to pay the agent and so on.
/// This demonstrates how the Reply message is handled.
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // The code below is copy/pasted from make_croncat_tick_task
    // and is deliberately duplicate.
    // If you're using this example as a template, feel free to remove this and tick_fail
    let croncat_factory_address = CRONCAT_FACTORY_ADDRESS.load(deps.storage)?;

    let croncat_task = CronCatTaskRequest {
        interval: CronCatInterval::Block(1),
        boundary: None,
        stop_on_fail: false, // So we can reproduce the error repeatedly
        actions: vec![CronCatAction {
            msg: Wasm(Execute {
                // Call "myself" at the tick method
                contract_addr: env.contract.address.to_string(),
                msg: to_binary(&ExecuteMsg::TickFail {})?,
                funds: vec![],
            }),
            gas_limit: Some(550_000), // can fine tune gas here
        }],
        queries: None,
        transforms: None,
        cw20: None,
    };

    let sub_message = create_croncat_task_submessage(
        &deps.querier,
        info,
        croncat_factory_address,
        croncat_task,
        None,
    )?;

    Ok(Response::new()
        .add_attribute("action", "make_croncat_tick_fail_task")
        .add_submessage(sub_message))
}
