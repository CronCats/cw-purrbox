pub mod make_croncat_toggle_task;

use crate::errors::ContractError;
use crate::msgs::execute_msg::ExecuteMsg;
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MakeCroncatToggleTask {} => make_croncat_toggle_task::execute(deps, env, info),
    }
}
