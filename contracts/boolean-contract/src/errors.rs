//! The errors that can be thrown for this boolean contract, including demonstration ones.

use cosmwasm_std::StdError;
use thiserror::Error;

/// List of common errors, including ones with arguments
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("This error will print a variable: {parameter1})")]
    SpecificError { parameter1: String },
}
