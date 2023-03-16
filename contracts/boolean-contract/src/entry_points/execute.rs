pub mod set_value;
pub mod toggle;

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
        ExecuteMsg::SetValue { is_true } => set_value::execute(deps, env, info, is_true)?,
        // Note you can also use opening and closing curly brackets
        // if you wanna have a whole block of code that does stuff
        ExecuteMsg::Toggle {} => {
            // Note we don't need to pass "is_true" in here cuz
            // we're just going to load and save the opposite value (true/false)
            toggle::execute(deps, env, info)?
        }
    };

    Ok(Response::default())
}
