use crate::errors::ContractError;
use cosmwasm_std::{DepsMut, Reply, Response};
use croncat_integration_utils::reply_handler::{
    reply_complete_task_creation, reply_handle_task_creation,
};
use croncat_integration_utils::CronCatTaskExecutionInfo;

pub fn reply(_deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    // Pass the reply message into a CronCat integration helper
    // This returns helpful information about the task including hash, owner, etc.
    let task_info_raw: CronCatTaskExecutionInfo = reply_handle_task_creation(msg)?;

    // YOUR CODE HERE

    // An optional helper method bringing slightly better ergonomics
    // so you don't end up having to base64 decode twice
    match reply_complete_task_creation(task_info_raw) {
        Ok(response) => Ok(response),
        Err(error) => Err(ContractError::from(error)),
    }
}
