#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, Order, MessageInfo, Reply, Response, StdResult, StdError, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw_croncat_core::types::RuleResponse;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, Basket, JunoswapExecuteMsg, JunoswapQueryMsg, Token1ForToken2PriceResponse};
use crate::state::{State, STATE, BASKETS};

// version info for migration info
const CONTRACT_NAME: &str = "cw-purrbox:dca-junoswap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const DCA_SWAP_REPLY_ID: u64 = 1u64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        callers: vec![info.sender.clone()],
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        DCA_SWAP_REPLY_ID => dca_swap_by_id_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

// TODO: REPLY
// Update the executed timestamp (& balance?!)
fn dca_swap_by_id_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    // let msg_unbinary: String = from_binary(&msg.result.unwrap())?;
    // // let msg_parsed: Value = serde_json::from_str(msg_unbinary);
    // let msg_parse = serde_json::from_str(msg_unbinary.as_str());

    // Save res.contract_address
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::DcaSwapById { id } => dca_swap_by_id(deps, env, info, id),
        ExecuteMsg::AddBasket { id, basket } => add_basket(deps, info, id, basket),
        ExecuteMsg::RemoveBasket { id } => remove_basket(deps, info, id),
        ExecuteMsg::AddCaller { caller } => add_caller(deps, info, caller),
        ExecuteMsg::RemoveCaller { caller } => remove_caller(deps, info, caller),
        ExecuteMsg::ChangeOwner { owner_id } => change_owner(deps, info, owner_id),
    }
}

pub fn dca_swap_by_id(deps: DepsMut, env: Env, info: MessageInfo, id: String) -> Result<Response, ContractError> {
    let b = BASKETS.may_load(deps.storage, id)?;
    if b.is_none() {
        return Err(ContractError::CustomError {
            val: "Basket doesnt exist".to_string(),
        });
    }
    let basket = b.unwrap();

    // Panic if the swap is happening too soon (_env)
    if basket.last_interval + basket.min_interval > env.block.height {
        return Err(ContractError::CustomError {
            val: "Too soon try later".to_string(),
        });
    }

    // Query the swap rate
    let valid_contract = deps.api.addr_validate(&basket.swap_address.to_string())?;
    let price_res: Token1ForToken2PriceResponse = deps.querier.query_wasm_smart(
        valid_contract,
        &JunoswapQueryMsg::Token1ForToken2Price {
            token1_amount: basket.swap_amount
        },
    )?;

    let swap = if let Some(recipient) = basket.recipient {
        JunoswapExecuteMsg::SwapAndSendTo {
            input_token: basket.input_token,
            input_amount: basket.swap_amount,
            min_token: price_res.token2_amount,
            recipient: recipient.to_string(),
            expiration: None,
        }
    } else {
        JunoswapExecuteMsg::Swap {
            input_token: basket.input_token,
            input_amount: basket.swap_amount,
            min_output: price_res.token2_amount,
            expiration: None,
        }
    };

    // Execute the swap via submsg
    let swap_msg = WasmMsg::Execute {
        contract_addr: basket.swap_address.to_string(),
        msg: to_binary(&swap)?,
        funds: vec![]
    };
    let submsg = SubMsg::reply_on_success(swap_msg.into(), DCA_SWAP_REPLY_ID);

    Ok(Response::new()
        .add_attribute("method", "dca_swap_by_id")
        .add_submessage(submsg))
}

pub fn add_basket(deps: DepsMut, info: MessageInfo, id: String, basket: Basket) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    BASKETS.update(deps.storage, id, |old| match old {
        Some(_) => Err(ContractError::CustomError {
            val: "Task already exists".to_string(),
        }),
        None => Ok(basket),
    })?;
    Ok(Response::new().add_attribute("method", "add_basket")
        .add_attribute("basket_id", id))
}

pub fn remove_basket(deps: DepsMut, info: MessageInfo, id: String) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    BASKETS.remove(deps.storage, id);
    Ok(Response::new().add_attribute("method", "remove_basket")
        .add_attribute("basket_id", id))
}

pub fn add_caller(deps: DepsMut, info: MessageInfo, caller: Addr) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.callers.push(caller);
    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "add_caller"))
}

pub fn remove_caller(deps: DepsMut, info: MessageInfo, caller: Addr) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    // Remove from the list of active agents if the agent in this list
    let mut callers: Vec<Addr> = state.callers;
    if let Some(index) = callers.iter().position(|addr| *addr == caller) {
        callers.remove(index);

        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.callers = callers;
            Ok(state)
        })?;
    }
    Ok(Response::new().add_attribute("method", "remove_caller"))
}

pub fn change_owner(deps: DepsMut, info: MessageInfo, owner_id: Addr) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    Ok(Response::new().add_attribute("method", "change_owner")
        .add_attribute("owner", owner_id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::HasSwapById { id } => to_binary(&query_has_swap_by_id(deps, id)?),
        QueryMsg::CanSwapById { id } => to_binary(&query_can_swap_by_id(deps, id)?),
        QueryMsg::GetBasketById { id } => to_binary(&query_get_basket_by_id(deps, id)?),
        QueryMsg::GetBasketIds {} => to_binary(&query_get_basket_ids(deps)?),
        QueryMsg::GetConfig {} => to_binary(&query_get_config(deps)?),
    }
}

/// TODO: Implements a method to check if a threshold is met, returning if a swap should be made
fn query_has_swap_by_id(deps: Deps, id: String) -> StdResult<RuleResponse<Option<String>>> {
    Ok((false, None))
}

/// TODO: Implements a method to check if the swap is actually ready, so no failed TXNs
fn query_can_swap_by_id(deps: Deps, id: String) -> StdResult<RuleResponse<Option<String>>> {
    Ok((false, None))
}

/// Returns a basket details
fn query_get_basket_by_id(deps: Deps, id: String) -> StdResult<Basket> {
    let item = BASKETS.load(deps.storage, id)?;
    Ok(item)
}

/// Returns basket ids
fn query_get_basket_ids(deps: Deps) -> StdResult<Vec<String>> {
    let baskets = BASKETS.keys(deps.storage, None, None, Order::Ascending).collect::<StdResult<Vec<_>>>()?;
    Ok(baskets)
}

/// Returns basket details
fn query_get_config(deps: Deps) -> StdResult<(Addr, Vec<Addr>)> {
    let state = STATE.load(deps.storage)?;
    Ok((state.owner, state.callers))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
//     use cosmwasm_std::{coins, from_binary};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let unauth_info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//         match res {
//             Err(ContractError::Unauthorized {}) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let auth_info = mock_info("creator", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
