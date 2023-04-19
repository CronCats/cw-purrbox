pub mod entry_points;
pub mod errors;
pub mod msgs;
pub mod state;
pub mod utils;

#[cfg(test)]
mod tests;

// Version info
pub const CONTRACT_NAME: &str = "crates.io:create-task-handle-tick";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const TEN_SECONDS_IN_NANOS: u64 = 10_000_000_000;
