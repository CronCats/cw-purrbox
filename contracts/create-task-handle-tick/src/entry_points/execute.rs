pub mod make_auctions;
pub mod make_croncat_tick_fail_task;
pub mod make_croncat_tick_task;
pub mod tick;
pub mod tick_fail;

use crate::errors::ContractError;
use crate::msgs::execute_msg::ExecuteMsg;
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MakeCroncatTickTask {} => make_croncat_tick_task::execute(deps, env, info),
        ExecuteMsg::MakeCroncatTickFailTask {} => {
            make_croncat_tick_fail_task::execute(deps, env, info)
        }
        ExecuteMsg::Tick {} => tick::execute(deps, env, info),
        ExecuteMsg::TickFail {} => tick_fail::execute(deps, env, info),
        ExecuteMsg::MakeAuctions {} => make_auctions::execute(deps, env, info),
    }
}
