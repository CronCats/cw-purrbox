use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Contract {0} versioner has already been registered on chain {1}")]
    ContractAlreadyRegistered(String, String),
    #[error("Contract {0} versioner not registered on chain {1}")]
    ContractNotRegistered(String, String),
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
