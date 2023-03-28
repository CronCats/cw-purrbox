use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    MakeCroncatToggleTask {},
    DemoLatestContracts {},
    DemoLatestContract { contract_name: String },
}
