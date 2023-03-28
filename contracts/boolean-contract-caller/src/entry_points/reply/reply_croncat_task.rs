use crate::errors::ContractError;
use crate::REPLY_CRONCAT_TASK_CREATION;
use cosmwasm_std::{DepsMut, Reply, Response, Uint64};

pub fn reply(_deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    if let Err(err) = msg.clone().result.into_result() {
        return Err(ContractError::ReplyError {
            reply_id: Uint64::from(REPLY_CRONCAT_TASK_CREATION),
            msg: format!("{:?}", err.clone()),
        });
    }

    // Let's take a look at the reply msg and save the task hash
    let raw_task_hash_opt = msg.result.unwrap().data;
    // This is one of those interesting cases in Rust where the variable
    // is mutable since it's not assigned a value.
    let task_hash: String;
    if let Some(raw_task_hash) = raw_task_hash_opt {
        task_hash = raw_task_hash.to_string();
    } else {
        return Err(ContractError::ReplyError {
            reply_id: Uint64::from(REPLY_CRONCAT_TASK_CREATION),
            msg: "Did not receive task hash".to_string(),
        });
    }

    // This is where you might save your task hash to internal state if you wish

    let resp = Response::new().add_attribute("task_hash", task_hash);

    Ok(resp)
}
