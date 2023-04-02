use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const REGISTRAR_ADDR: Item<Addr> = Item::new("registrar");
pub const CRONCAT_ADDR: Item<Addr> = Item::new("croncat");
pub const VERSION_MAP: Map<(&str, &str), String> = Map::new("version_map");
