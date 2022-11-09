#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, coins, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Order, Reply,
    Response, StdError, StdResult, SubMsg, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw_croncat_core::types::RuleResponse;

use crate::error::ContractError;
use crate::msg::{
    Basket, ExecuteMsg, InstantiateMsg, JunoswapExecuteMsg, JunoswapQueryMsg, QueryMsg,
    Token1ForToken2PriceResponse,
};
use crate::state::{State, BASKETS, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "cw-purrbox:dca-junoswap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const DCA_SWAP_REPLY_ID: u64 = 1u64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
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

fn dca_swap_by_id_reply(_deps: DepsMut, _msg: Reply) -> StdResult<Response> {
    // let msg_unbinary: String = from_binary(&msg.result.unwrap().data.unwrap())?;
    // // let msg_parsed: Value = serde_json::from_str(msg_unbinary);
    // let msg_parse = serde_json::from_str(msg_unbinary.as_str());

    Ok(Response::new().add_attribute("method", "dca_swap_by_id_reply"))
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
        ExecuteMsg::AddBasket { id, basket } => add_basket(deps, info, id, *basket),
        ExecuteMsg::RefillBasket { id } => refill_basket(deps, info, id),
        ExecuteMsg::RemoveBasket { id } => remove_basket(deps, info, id),
        ExecuteMsg::AddCaller { caller } => add_caller(deps, info, caller),
        ExecuteMsg::RemoveCaller { caller } => remove_caller(deps, info, caller),
        ExecuteMsg::ChangeOwner { owner_id } => change_owner(deps, info, owner_id),
    }
}

pub fn dca_swap_by_id(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    // only allow known callers
    let state = STATE.load(deps.storage)?;
    if !state.callers.contains(&info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let b = BASKETS.may_load(deps.storage, id.clone())?;
    if b.is_none() {
        return Err(ContractError::CustomError {
            val: "Basket doesnt exist".to_string(),
        });
    }
    let basket = b.unwrap();

    // Panic if the swap is happening too soon (_env)
    if basket
        .clone()
        .last_interval
        .unwrap_or_default()
        .saturating_add(basket.clone().min_interval.unwrap_or(100))
        > env.block.height
    {
        return Err(ContractError::CustomError {
            val: "Too soon try later".to_string(),
        });
    }

    // If balance is too low need to FWD funds to recipient then end
    let bal_amt: u128 = basket.clone().balance.amount.into();
    let swap_amt: u128 = basket.clone().swap_amount.into();
    let bal_denom = basket.clone().balance.denom;
    let state = STATE.load(deps.storage)?;

    if swap_amt > bal_amt {
        let refund_addr: Addr = if let Some(recipient) = basket.clone().recipient {
            recipient
        } else {
            state.owner
        };

        let bank_send = BankMsg::Send {
            to_address: refund_addr.to_string(),
            amount: coins(bal_amt, bal_denom),
        };

        return Ok(Response::new()
            .add_attribute("method", "dca_swap_by_id")
            .add_attribute("type", "end_refund_basket")
            .add_message(bank_send));
    }

    // Query the swap rate
    let valid_contract = deps.api.addr_validate(&basket.swap_address.to_string())?;
    let price_res: Token1ForToken2PriceResponse = deps.querier.query_wasm_smart(
        valid_contract.clone(),
        &JunoswapQueryMsg::Token1ForToken2Price {
            token1_amount: Uint128::from(swap_amt),
        },
    )?;

    // swap rate checks (IF specified)
    if basket.min_swap_rate.is_some() {
        if let Some(last_swap) = basket.last_swap {
            // For now, we're really only going 1 direction, so use second value
            let ready = validate_swap_threshold(
                price_res.token2_amount,
                last_swap[1],
                basket.min_swap_rate.unwrap_or_default(),
            );

            if !ready {
                return Err(ContractError::CustomError {
                    val: "Swap rate out of bounds".to_string(),
                });
            }
        }
    }

    // Update balance
    BASKETS.update(deps.storage, id, |old| match old {
        Some(_) => {
            let mut o = old.unwrap();
            let chg_amt = bal_amt.saturating_sub(swap_amt);
            o.balance = coin(chg_amt, basket.clone().balance.denom);
            o.last_interval = Some(env.block.height);
            o.last_swap = Some([swap_amt.into(), price_res.token2_amount]);

            Ok(o)
        }
        None => Err(ContractError::CustomError {
            val: "Basket doesnt exist".to_string(),
        }),
    })?;

    // recipient makes
    let swap = if let Some(recipient) = basket.clone().recipient {
        JunoswapExecuteMsg::SwapAndSendTo {
            input_token: basket.clone().input_token,
            input_amount: Uint128::from(swap_amt),
            min_token: price_res.token2_amount,
            recipient: recipient.to_string(),
            expiration: None,
        }
    } else {
        JunoswapExecuteMsg::Swap {
            input_token: basket.clone().input_token,
            input_amount: Uint128::from(swap_amt),
            min_output: price_res.token2_amount,
            expiration: None,
        }
    };

    // Execute the swap via submsg
    let swap_msg = WasmMsg::Execute {
        contract_addr: valid_contract.to_string(),
        msg: to_binary(&swap)?,
        funds: coins(swap_amt, bal_denom),
    };
    let submsg = SubMsg::reply_on_success(swap_msg, DCA_SWAP_REPLY_ID);

    Ok(Response::new()
        .add_attribute("method", "dca_swap_by_id")
        .add_submessage(submsg))
}

pub fn add_basket(
    deps: DepsMut,
    info: MessageInfo,
    id: String,
    basket: Basket,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    BASKETS.update(deps.storage, id.clone(), |old| match old {
        Some(_) => Err(ContractError::CustomError {
            val: "Basket already exists".to_string(),
        }),
        None => Ok(basket),
    })?;
    Ok(Response::new()
        .add_attribute("method", "add_basket")
        .add_attribute("basket_id", id))
}

pub fn refill_basket(
    deps: DepsMut,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    BASKETS.update(deps.storage, id.clone(), |b| match b {
        Some(mut b) => {
            // only find matching funds
            let c = b.clone().balance;

            for fund in info.funds.iter() {
                if fund.denom == c.clone().denom {
                    let chg_amt = b.balance.amount.saturating_add(fund.amount);
                    b.balance = coin(chg_amt.into(), c.clone().denom);
                }
            }

            Ok(b)
        }
        None => Err(ContractError::CustomError {
            val: "Basket doesnt exist".to_string(),
        }),
    })?;
    Ok(Response::new()
        .add_attribute("method", "add_basket")
        .add_attribute("basket_id", id))
}

pub fn remove_basket(
    deps: DepsMut,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    // TODO: refund any remaining balance(s)??
    BASKETS.remove(deps.storage, id.clone());
    Ok(Response::new()
        .add_attribute("method", "remove_basket")
        .add_attribute("basket_id", id))
}

pub fn add_caller(
    deps: DepsMut,
    info: MessageInfo,
    caller: Addr,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.callers.push(caller);
    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "add_caller"))
}

pub fn remove_caller(
    deps: DepsMut,
    info: MessageInfo,
    caller: Addr,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
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

pub fn change_owner(
    deps: DepsMut,
    info: MessageInfo,
    owner_id: Addr,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    Ok(Response::new()
        .add_attribute("method", "change_owner")
        .add_attribute("owner", owner_id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::HasSwapById { id } => to_binary(&query_has_swap_by_id(deps, id)?),
        QueryMsg::CanSwapById { id } => to_binary(&query_can_swap_by_id(deps, env, id)?),
        QueryMsg::GetBasketById { id } => to_binary(&query_get_basket_by_id(deps, id)?),
        QueryMsg::GetBasketIds {} => to_binary(&query_get_basket_ids(deps)?),
        QueryMsg::GetConfig {} => to_binary(&query_get_config(deps)?),
    }
}

fn validate_swap_threshold(
    curr_amount: Uint128,
    prev_amount: Uint128,
    min_swap_rate: Uint128,
) -> bool {
    let offset_amount = Uint128::from(
        u128::from(prev_amount)
            .checked_mul(u128::from(min_swap_rate))
            .and_then(|n| n.checked_div(100))
            .and_then(|n| n.checked_add(u128::from(prev_amount)))
            .unwrap(),
    );

    curr_amount > prev_amount && curr_amount > offset_amount
}

/// Check if a threshold is met, returning if a swap should be made
fn query_has_swap_by_id(deps: Deps, id: String) -> StdResult<RuleResponse<Option<String>>> {
    let b = BASKETS.may_load(deps.storage, id)?;
    if b.is_none() {
        return Ok((false, None));
    }
    let basket = b.unwrap();
    let swap_amt: u128 = basket.swap_amount.into();

    // Only if swaps occurred!
    if let Some(last_swap) = basket.last_swap {
        // Query the swap rate
        let valid_contract = deps.api.addr_validate(&basket.swap_address.to_string())?;
        let price_res: Token1ForToken2PriceResponse = deps.querier.query_wasm_smart(
            valid_contract,
            &JunoswapQueryMsg::Token1ForToken2Price {
                token1_amount: Uint128::from(swap_amt),
            },
        )?;

        // For now, we're really only going 1 direction, so use second value
        let ready = validate_swap_threshold(
            price_res.token2_amount,
            last_swap[1],
            basket.min_swap_rate.unwrap_or_default(),
        );

        return Ok((ready, None));
    }

    // No swaps yet, so GO GO GO! Gets a baseline for next swap
    // Note: If this was matched with a TWAP, would help accuracy
    Ok((true, None))
}

/// Check if the swap is actually ready, so no failed TXNs
fn query_can_swap_by_id(
    deps: Deps,
    env: Env,
    id: String,
) -> StdResult<RuleResponse<Option<String>>> {
    let b = BASKETS.may_load(deps.storage, id)?;
    if b.is_none() {
        return Ok((false, None));
    }
    let basket = b.unwrap();

    // Panic if the swap is happening too soon (_env)
    if basket
        .last_interval
        .unwrap_or_default()
        .saturating_add(basket.min_interval.unwrap_or(100))
        > env.block.height
    {
        return Ok((false, None));
    }

    Ok((true, None))
}

/// Returns a basket details
fn query_get_basket_by_id(deps: Deps, id: String) -> StdResult<Basket> {
    let item = BASKETS.load(deps.storage, id)?;
    Ok(item)
}

/// Returns basket ids
fn query_get_basket_ids(deps: Deps) -> StdResult<Vec<String>> {
    let baskets = BASKETS
        .keys(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;
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
