use crate::state::{Auction, MOCK_AUCTIONS};
use cosmwasm_std::{Deps, Env, StdResult};

pub fn query(deps: Deps, _env: Env) -> StdResult<Vec<Auction>> {
    Ok(MOCK_AUCTIONS.load(deps.storage)?)
}
