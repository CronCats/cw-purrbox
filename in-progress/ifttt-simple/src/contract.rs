#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use cw_croncat_core::types::RuleResponse;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ifttt-simple";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
    }
}

pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::CheckModulo {} => to_binary(&check_modulo(deps, env)?),
        QueryMsg::CheckInputModulo { msg } => to_binary(&check_input_modulo(msg)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}

// Return TRUE if EVEN, FALSE if ODD
// RuleResponse is used for additional data passed back
fn check_modulo(deps: Deps, env: Env) -> StdResult<RuleResponse<Option<Binary>>> {
    // modulo check if even, block level
    let b: bool = env.block.height % 2 == 0;

    // If TRUE then return some interesting sample data
    let data = if b {
        let state = STATE.load(deps.storage)?;
        Some(to_binary(&CountResponse { count: state.count })?)
    } else {
        None
    };

    Ok((b, data))
}

// Return TRUE if modulo of count is EVEN
// msg is a Binary here, to show how interpretting can be done,
// this example shows binary passed from above "check_modulo"
// But could be anything or another msg matcher
// RuleResponse is used for additional data passed back
fn check_input_modulo(msg: Binary) -> StdResult<RuleResponse<Option<Binary>>> {
    let msg_value: CountResponse = from_binary(&msg)?;

    // modulo check if even, from the passed in variable
    let b: bool = msg_value.count % 2 == 0;

    Ok((b, None))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn single_modulo_check() {
        let mut env = mock_env();
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Check if false
        let res = query(deps.as_ref(), env.clone(), QueryMsg::CheckModulo {}).unwrap();
        let (b, _d): RuleResponse<Option<Binary>> = from_binary(&res).unwrap();
        assert_eq!(b, false);

        env.block.height += 1;

        // Check if true
        let res = query(deps.as_ref(), env.clone(), QueryMsg::CheckModulo {}).unwrap();
        let (b, _d): RuleResponse<Option<Binary>> = from_binary(&res).unwrap();
        assert_eq!(b, true);

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn multi_modulo_check() {
        let mut env = mock_env();
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Check if false
        let res = query(deps.as_ref(), env.clone(), QueryMsg::CheckModulo {}).unwrap();
        let (b, _d): RuleResponse<Option<Binary>> = from_binary(&res).unwrap();
        assert_eq!(b, false);

        env.block.height += 1;

        // Check if true
        let res = query(deps.as_ref(), env.clone(), QueryMsg::CheckModulo {}).unwrap();
        let (b, d): RuleResponse<Option<Binary>> = from_binary(&res).unwrap();
        assert_eq!(b, true);

        // Check if modulo of previous RES is then false
        let res = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::CheckInputModulo {
                msg: d.clone().unwrap(),
            },
        )
        .unwrap();
        let (b, _d): RuleResponse<Option<Binary>> = from_binary(&res).unwrap();
        assert_eq!(b, false);

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), env.clone(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);

        // Check if modulo is true again, because it got incremented
        let res = query(deps.as_ref(), env.clone(), QueryMsg::CheckModulo {}).unwrap();
        let (b, dd): RuleResponse<Option<Binary>> = from_binary(&res).unwrap();
        assert_eq!(b, true);

        // Check if modulo of previous RES is then true
        let res = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::CheckInputModulo { msg: dd.unwrap() },
        )
        .unwrap();
        let (b, _d): RuleResponse<Option<Binary>> = from_binary(&res).unwrap();
        assert_eq!(b, true);
    }

    // ---- Not interesting tests below ----

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
