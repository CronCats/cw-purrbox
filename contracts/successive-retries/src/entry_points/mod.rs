//! CosmWasm smart contracts contain entry points.
//! More on this pattern here: [https://book.cosmwasm.com/basics/entry-points.html](https://book.cosmwasm.com/basics/entry-points.html)

/// The execute entry point
pub mod execute;
/// The instantiate entry point
pub mod instantiate;
/// The query entry point
pub mod query;
