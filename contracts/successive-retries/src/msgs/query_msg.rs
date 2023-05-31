use cosmwasm_schema::{cw_serde, QueryResponses};

/// The available queries, which is only `failed_attempts`.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u8)]
    FailedAttempts {}, // No parameters needed
}
