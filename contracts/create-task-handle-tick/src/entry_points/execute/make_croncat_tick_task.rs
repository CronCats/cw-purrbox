use crate::errors::ContractError;
use crate::msgs::execute_msg::ExecuteMsg;
use crate::state::CRONCAT_FACTORY_ADDRESS;
use cosmwasm_std::CosmosMsg::Wasm;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response};
use croncat_integration_utils::task_creation::create_croncat_task_submessage;
use croncat_integration_utils::{CronCatAction, CronCatInterval, CronCatTaskRequest};

/// Create a CronCat task using cross-contract calls (submessages)
/// Take a look in `reply_croncat_task` to see how we gather
/// information on the newly-created task, including whether
/// it was successful or not.
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // Load the CronCat factory address you've saved to state.
    let croncat_factory_address = CRONCAT_FACTORY_ADDRESS.load(deps.storage)?;

    // Create a task that fires every block, stops and refunds remaining
    // funds if the call fails.
    // This will have one Action: to call myself at the "tick" method.
    let croncat_task = CronCatTaskRequest {
        interval: CronCatInterval::Block(1),
        boundary: None,
        stop_on_fail: true,
        actions: vec![CronCatAction {
            msg: Wasm(Execute {
                contract_addr: env.contract.address.to_string(),
                msg: to_binary(&ExecuteMsg::Tick {})?,
                funds: vec![],
            }),
            gas_limit: Some(550_000), // Can fine tune gas here
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
        .add_attribute("action", "make_croncat_tick_task")
        .add_submessage(sub_message))
}
