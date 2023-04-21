#[allow(unused)]
use crate::state::Auction;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<Auction>)]
    Auctions {},
}
