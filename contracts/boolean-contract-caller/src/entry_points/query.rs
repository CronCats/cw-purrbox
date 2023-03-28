use crate::msgs::query_msg::QueryMsg;
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, Env, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // There are no queries in this simple example
    // But you're supposed to add a few entry points (including query) in every contract
    match msg {}
}
