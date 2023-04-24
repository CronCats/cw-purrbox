use cosmwasm_std::{StdError, Uint64};
use croncat_errors_macro::croncat_error;
use thiserror::Error;

// CRONCAT HELPER
// Note: you'll want to place this macro above the derive.
// It'll throw a helper error if you forget.
#[croncat_error]
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Reply error|{reply_id}|{msg}")]
    ReplyError { reply_id: Uint64, msg: String },

    #[error("{code:?}|{msg:?}")]
    CustomError { code: String, msg: String },

    #[error("Unknown reply ID|Unknown reply ID: {id:?}")]
    UnknownReplyID { id: Uint64 },
}
