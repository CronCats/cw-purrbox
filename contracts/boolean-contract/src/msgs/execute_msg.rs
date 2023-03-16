use cosmwasm_schema::cw_serde;

/// Lists the available execute messages for this contract
#[cw_serde]
pub enum ExecuteMsg {
    /// Explicitly sets the state boolean to `true` or `false`
    /// Takes one parameter: `is_true` and uses that value to set state
    SetValue { is_true: bool },
    /// Toggle switches the current value in state from `true` to `false` or `false` to `true`
    /// This execute message takes no parameters
    Toggle {},
}
