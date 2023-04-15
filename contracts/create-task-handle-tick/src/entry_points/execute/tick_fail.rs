use crate::errors::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cw_serde]
pub enum NewManagerMethods {
    LatestTaskExecution {},
}

/// This method intentionally fails, demonstrating the effects
/// of a CronCat task executing an Action and failing.
/// If the task set `stop_on_fail` to `true`, the task will be
/// removed.
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    // This method is meant to fail and show what happens
    // If you're using this example as a template, feel free to remove this and make_croncat_tick_fail_task
    Err(ContractError::CustomError {
        code: "INTENTIONAL_FAILURE".to_string(),
        msg: "Demonstrating what happens during a failure".to_string(),
    })
}
