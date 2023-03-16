pub mod entry_points;
pub mod msgs;
pub mod errors;
pub mod state;
pub mod utils;
mod tests;

// Version info
pub const CONTRACT_NAME: &str = "crates.io:cw-boolean-contract";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Reply ID
pub const REPLY_CRONCAT_TASK_CREATION: u64 = 0;
pub const CRONCAT_FACTORY_ADDRESS: &[u8] = "factory".as_bytes();
pub const BOOLEAN_ADDRESS: &[u8] = "boolean".as_bytes();