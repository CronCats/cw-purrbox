use cosmwasm_schema::{cw_serde, QueryResponses};
#[allow(unused)]
use crate::state::Auction;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
  #[returns(Vec<Auction>)]
  Auctions {}
}
