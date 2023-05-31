//! What's stored in the contract state. For this simple contract, that's just a boolean.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

/// The state that's stored.
/// Keeps track of how many retries have been attempted.
#[cw_serde]
pub struct Config {
    pub number_of_tries: u8,
    pub denom: String,
    pub max_times: u8,
    pub delay_in_minutes: u8,
}

// We might as well shorten it to "c" instead of "config"
/// We're using [`cw-storage-plus`](https://crates.io/crates/cw-storage-plus)'s [`Item`](cw_storage_plus::Item)
pub const CONFIG: Item<Config> = Item::new("c");

pub const CRONCAT_FACTORY_ADDRESS: Item<Addr> = Item::new("cfa");
pub const PUBLIC_FUNDING_ADDRESS: Item<Addr> = Item::new("pfa");
/// In this example, we'll set this to double whatever amount this contract started with.
/// (See instantiate entry point)
pub const FUNDS_NECESSARY: Item<Uint128> = Item::new("fn");
