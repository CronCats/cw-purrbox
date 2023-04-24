use crate::errors::ContractError;
use crate::state::{BOOLEAN_ADDRESS, CRONCAT_FACTORY_ADDRESS};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::CosmosMsg::Wasm;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response};
use croncat_integration_utils::task_creation::create_croncat_task_submessage;
use croncat_integration_utils::{CronCatAction, CronCatInterval, CronCatTaskRequest};

// Let's say we don't have a package/crate but know the structure
// It's fine to define it here if you must.
#[cw_serde]
pub enum BooleanContractExecuteMsg {
    Toggle {},
}

pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let croncat_factory_address = CRONCAT_FACTORY_ADDRESS.load(deps.storage)?;
    let boolean_address = BOOLEAN_ADDRESS.load(deps.storage)?;

    let croncat_task = CronCatTaskRequest {
        interval: CronCatInterval::Block(1),
        boundary: None,
        stop_on_fail: true,
        actions: vec![CronCatAction {
            msg: Wasm(Execute {
                contract_addr: boolean_address.to_string(),
                msg: to_binary(&BooleanContractExecuteMsg::Toggle {})?,
                funds: vec![],
            }),
            gas_limit: Some(550_000), // can fine tune gas here
        }],
        queries: None,
        transforms: None,
        cw20: None,
    };

    // CRONCAT HELPER
    let sub_message = create_croncat_task_submessage(
        &deps.querier,
        info,
        croncat_factory_address,
        croncat_task,
        None,
    )?;

    Ok(Response::new()
        .add_attribute("action", "make_croncat_toggle_task")
        .add_submessage(sub_message))
}
