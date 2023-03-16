//! Execute logic that inverts the current stored boolean value.

use crate::errors::ContractError;
use crate::state::{Config, CONFIG};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

/// Logic for the [Toggle](crate::msgs::execute_msg::ExecuteMsg::Toggle) (`toggle`) method
pub fn execute(deps: DepsMut, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    // Get the current value
    let current_val = CONFIG.load(deps.storage)?;
    let toggle_boolean = Config {
        // The exclamation point says, "the opposite of the true/false, please"
        is_true: !current_val.is_true,
    };
    CONFIG.save(deps.storage, &toggle_boolean)?;
    Ok(Response::default())
}
