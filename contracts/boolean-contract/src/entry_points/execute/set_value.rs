//! Execute logic that explicitly sets the state boolean to `true` or `false`

use crate::errors::ContractError;
use crate::state::{Config, CONFIG};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

/// Logic for the [SetValue](crate::msgs::execute_msg::ExecuteMsg::SetValue) (`set_value`) method
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    is_true: bool,
) -> Result<Response, ContractError> {
    // Set our state variable according to the input
    let update_config = Config { is_true };
    // The question mark at the end of this line means it'll return an error
    // if something went wrong
    CONFIG.save(deps.storage, &update_config)?;

    // We basically say, "yup that worked" but returning Ok(…)
    // It's also best practice to use Response::default()
    // instead of Response::new() even though that would ✌️work ✌️
    Ok(Response::default())
}
