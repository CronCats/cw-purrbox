use cosmwasm_schema::cw_serde;

/// The instantiate message where you give the CronCat factory address
/// See <https://docs.cron.cat/docs/deployed-contracts> for networks and addresses.
#[cw_serde]
pub struct InstantiateMsg {
    /// CronCat factory address that keeps track of CronCat child contracts
    pub croncat_factory_address: String,
    /// The public funding DAO address for this example.
    /// If funds are received at this contract before the maximum retries,
    /// it will be sent to this address.
    pub public_funding_address: String,
    /// How many minutes the first delay is
    pub delay_in_minutes: u8,
    /// How many times to try
    pub max_times_to_try: u8,
}
