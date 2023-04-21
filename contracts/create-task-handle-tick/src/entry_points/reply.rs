mod reply_croncat_task;

use crate::errors::ContractError;
use cosmwasm_std::{entry_point, DepsMut, Env, Reply, Response};
use croncat_integration_utils::REPLY_CRONCAT_TASK_CREATION;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_CRONCAT_TASK_CREATION => reply_croncat_task::reply(deps, msg),
        id => Err(ContractError::UnknownReplyID { id }),
    }
}
