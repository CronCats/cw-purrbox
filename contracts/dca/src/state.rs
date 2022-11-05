use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::Basket;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub callers: Vec<Addr>,
}

pub const STATE: Item<State> = Item::new("state");

// key: pool_id, value: Basket
pub const BASKETS: Map<String, Basket> = Map::new("baskets");
