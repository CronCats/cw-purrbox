use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const CRONCAT_FACTORY_ADDRESS: Item<Addr> = Item::new("cfa");
pub const BOOLEAN_ADDRESS: Item<Addr> = Item::new("ba");
