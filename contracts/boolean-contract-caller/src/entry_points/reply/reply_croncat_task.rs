use cosmwasm_std::{DepsMut, Reply, Response};
use crate::errors::ContractError;
use crate::REPLY_CRONCAT_TASK_CREATION;

pub fn reply(_deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    // shouldn't happen because we used reply_on_success
    if let Err(err) = msg.clone().result.into_result() {
        Err(ContractError::ReplyError {
            code: REPLY_CRONCAT_TASK_CREATION,
            msg: format!("{:?}", err.clone()),
        })
    } else {
        Err(ContractError::CustomError {
            code: "I_DUNNO_BRO".to_string(),
            msg: "This is a custom error that happens when a reply is received for an error but it's not an error or something".to_string(),
        })
    }
}
