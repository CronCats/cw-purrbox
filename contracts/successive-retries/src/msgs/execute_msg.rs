use cosmwasm_schema::cw_serde;

/// Lists the available execute messages for this contract
#[cw_serde]
pub enum ExecuteMsg {
    /// This will be called by a CronCat task, and check for
    /// expected funds. If it's there, proceed to do business logic.
    /// If not, retry if we haven't reached the maximum retries.
    CheckFundsDoThing {},
}
