//! The errors that can be thrown for this boolean contract, including demonstration ones.

use cosmwasm_std::StdError;
use croncat_errors_macro::croncat_error;
use thiserror::Error;

// CRONCAT HELPER
// Note: you'll want to place this macro above the derive.
// It'll throw a helper error if you forget.
#[croncat_error]
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Can't say we've achieved the goal yet. :/")]
    GoalNotMet,
    /// Whether or not the goal was reached, after the retries it's completed.
    #[error("Completed")]
    Completed,
    #[error("Did not attach funds. Please add --amount 600000uosmo or something.")]
    NoFunds,
}
