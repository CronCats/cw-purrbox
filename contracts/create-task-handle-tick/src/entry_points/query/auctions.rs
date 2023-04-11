use cosmwasm_std::{Deps, Env, StdResult};
use crate::state::{Auction, MOCK_AUCTIONS};

pub fn query(deps: Deps, _env: Env) -> StdResult<Vec<Auction>> {
  Ok(MOCK_AUCTIONS.load(deps.storage)?)
}