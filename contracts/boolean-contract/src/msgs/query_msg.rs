use cosmwasm_schema::{cw_serde, QueryResponses};

/// The available queries, which is only `get_value`.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(bool)]
    GetValue {}, // No parameters needed
}
