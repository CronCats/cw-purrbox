use crate::state::CONFIG;
use cosmwasm_std::{Deps, Env, StdResult};

/// Logic for the [GetValue](crate::msgs::query_msg::QueryMsg::GetValue) (`get_value`) method
pub fn query(deps: Deps, _env: Env) -> StdResult<u8> {
    // Set our state variable according to the input
    let config = CONFIG.load(deps.storage)?;

    Ok(config.number_of_tries)
}
