//! Messages are how interactions occur in CosmWasm contracts.

/// Available execute messages (likely changing state)
pub mod execute_msg;
/// Available instantiate messages (the parameters sent to the bytecode when instantiating)
pub mod instantiate_msg;
/// Available query messages (read-only, not changing state)
pub mod query_msg;
