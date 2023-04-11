use crate::errors::ContractError;
use crate::REPLY_CRONCAT_TASK_CREATION;
use cosmwasm_std::{DepsMut, Reply, Response, Uint64};
use croncat_sdk_tasks::types::TaskExecutionInfo;
use cw_utils::parse_reply_execute_data;

pub fn reply(_deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    if let Err(err) = msg.clone().result.into_result() {
        return Err(ContractError::ReplyError {
            reply_id: REPLY_CRONCAT_TASK_CREATION.into(),
            msg: format!("{:?}", err.clone()),
        });
    }

    if let Err(err) = msg.clone().result.into_result() {
        return Err(ContractError::ReplyError {
            reply_id: Uint64::from(REPLY_CRONCAT_TASK_CREATION),
            msg: format!("{:?}", err.clone()),
        });
    }

    let msg_parsed = parse_reply_execute_data(msg);
    let msg_binary = msg_parsed.unwrap().data.unwrap();

    let created_task_info_res = serde_json_wasm::from_slice(msg_binary.clone().as_slice());

    if created_task_info_res.is_err() {
        return Err(ContractError::ReplyError {
            reply_id: Uint64::from(REPLY_CRONCAT_TASK_CREATION),
            msg: "Failed to decode reply data".to_string(),
        });
    }

    let created_task_info: TaskExecutionInfo = created_task_info_res.unwrap();

    // Here's where you could store the newly-created task details
    // in your contract's state if you wish.
    // Please see the create-task-handle-tick example for info.

    let task_info_json_vector = serde_json::to_vec(&created_task_info).unwrap();

    Ok(Response::new().set_data(&*task_info_json_vector))
}
