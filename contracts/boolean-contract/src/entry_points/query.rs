pub mod get_value;

use crate::msgs::query_msg::QueryMsg;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};

/// Query entry point
/// See a list of query variants in the [QueryMsg](QueryMsg) enum
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // inside this match, we list all the queries we have in this contract
    // we only have one, GetValue, which is turned into snake case,
    // so contracts and end users will call "get_value"
    let res = match msg {
        QueryMsg::GetValue {} => get_value::query(deps, env)?,
    };

    to_binary(&res)
}
