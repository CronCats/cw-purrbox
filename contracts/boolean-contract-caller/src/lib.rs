pub mod entry_points;
pub mod errors;
pub mod msgs;
pub mod state;

#[cfg(test)]
mod tests;

// Version info
pub const CONTRACT_NAME: &str = "crates.io:cw-boolean-contract";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
