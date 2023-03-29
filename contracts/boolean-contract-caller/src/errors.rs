use cosmwasm_std::{StdError, Uint64};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("ERR_REPLY_ERROR|{reply_id}|{msg}")]
    ReplyError { reply_id: Uint64, msg: String },

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Must attach funds when calling this method. All funds will be sent to the CronCat Task contract during task creation.")]
    NoFundsAttached {},

    #[error("{code:?}|{msg:?}")]
    CustomError { code: String, msg: String },

    #[error("ERR_UNKNOWN_REPLY|Unknown reply ID: {reply_id:?}")]
    UnknownReplyID { reply_id: Uint64 },
}
