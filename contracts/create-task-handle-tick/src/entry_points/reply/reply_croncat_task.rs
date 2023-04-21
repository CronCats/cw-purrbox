use crate::errors::ContractError;
use cosmwasm_std::{DepsMut, Reply, Response};
// CRONCAT HELPER
use croncat_integration_utils::reply_handler::reply_handle_croncat_task_creation;

pub fn reply(_deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    // Pass the reply message into a CronCat integration helper
    // This returns helpful information about the task including hash, owner, etc.
    let (_task_info, msg_binary) = reply_handle_croncat_task_creation(msg)?;

    // YOUR CODE HERE

    Ok(Response::new().set_data(msg_binary))
}
