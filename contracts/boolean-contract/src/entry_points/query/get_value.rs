use crate::state::CONFIG;
use cosmwasm_std::{Binary, Deps, Env, StdError, StdResult};
use mod_sdk::types::QueryResponse;

/// Logic for the [GetValue](crate::msgs::query_msg::QueryMsg::GetValue) (`get_value`) method
pub fn query(deps: Deps, _env: Env) -> StdResult<QueryResponse> {
    // Set our state variable according to the input
    let config = CONFIG.load(deps.storage);

    match config {
        Ok(c) => Ok(QueryResponse {
            result: c.is_true,
            data: Binary::default(),
        }),
        Err(_) => Err(StdError::generic_err(
            "Could not load config which has the boolean",
        )),
    }
}
