pub mod auctions;

use crate::msgs::query_msg::QueryMsg;
use cosmwasm_std::{entry_point, to_binary};
use cosmwasm_std::{Binary, Deps, Env, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Auctions { .. } => to_binary(&auctions::query(deps, env)?),
    }
}
