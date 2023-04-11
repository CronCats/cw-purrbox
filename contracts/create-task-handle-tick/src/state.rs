use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cosmwasm_std::Timestamp;
use cw_storage_plus::Item;

pub const CRONCAT_FACTORY_ADDRESS: Item<Addr> = Item::new("cfa");
pub const MOCK_AUCTIONS: Item<Vec<Auction>> = Item::new("ma");

#[cw_serde]
pub struct Auction {
    pub end_time: Timestamp,
    pub title: String,
}
