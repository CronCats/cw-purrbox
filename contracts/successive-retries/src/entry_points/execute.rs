pub mod check_funds_do_thing;

use crate::errors::ContractError;
use crate::msgs::execute_msg::ExecuteMsg;
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

/// Execute entry point.
/// You may see a list of the execute variants (methods) in [ExecuteMsg](ExecuteMsg)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CheckFundsDoThing {} => check_funds_do_thing::execute(deps, env, info),
    }
}
