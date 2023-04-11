use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    MakeAuctions {},
    MakeCroncatTickTask {},
    MakeCroncatTickFailTask {},
    Tick {},
    TickFail {},
}
