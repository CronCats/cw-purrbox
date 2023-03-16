#![doc(html_logo_url = "https://docs.cron.cat/icons/icon-512x512.png")]

#![allow(rustdoc::broken_intra_doc_links)]
//! For an overview, see the [README here](readme), since Rust docs are odd.

pub mod entry_points;
pub mod errors;
pub mod msgs;
pub mod state;

#[cfg(test)]
mod tests;

/// Silly way to have a README and the docs how I like it.
// #[cfg(test)]
pub mod readme;

/// Contract name as it'll be stored according to the [cw2 dependency](https://crates.io/crates/cw2)
pub const CONTRACT_NAME: &str = "crates.io:cw-boolean-contract";
/// The version comes from the manifest file (Cargo.toml)
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
