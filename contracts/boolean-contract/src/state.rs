//! What's stored in the contract state. For this simple contract, that's just a boolean.

use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

/// The state that's stored.
/// Simply an `is_true` key of type `boolean`.
#[cw_serde]
pub struct Config {
    pub is_true: bool,
}

// We might as well shorten it to "c" instead of "config"
/// We're using [`cw-storage-plus`](https://crates.io/crates/cw-storage-plus)'s [`Item`](cw_storage_plus::Item)
pub const CONFIG: Item<Config> = Item::new("c");
