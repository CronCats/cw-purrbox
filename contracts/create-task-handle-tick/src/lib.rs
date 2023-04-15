pub mod entry_points;
pub mod errors;
pub mod msgs;
pub mod state;
pub mod utils;

#[cfg(test)]
mod tests;

// Version info
pub const CONTRACT_NAME: &str = "crates.io:cw-boolean-contract";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Reply ID
pub const REPLY_CRONCAT_TASK_CREATION: u64 = 0;

// pub const MINUTE_IN_NANOS: u64 = 60_000_000_000;
pub const TEN_SECONDS_IN_NANOS: u64 = 10_000_000_000;
