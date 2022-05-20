use oorandom::Rand32;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, SubMsg, WasmQuery,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg, QueryTestResponse, RandomResponse, RuleResponse,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub msg_cosmos: CosmosMsg,
    pub msg_sub: SubMsg,
    pub msg_binary: Binary,
    pub msg_query: WasmQuery,
}

pub const STATE: Item<State> = Item::new("state");

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:multi-query";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::QueryResult {} => query_result(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRandom {} => to_binary(&query_random(env)?),
        QueryMsg::GetBoolBinary { msg } => to_binary(&query_bool_binary_echo(msg)?),
        QueryMsg::GetInputBoolBinary { msg } => to_binary(&query_input_bool_binary(msg)?),
        QueryMsg::QueryChain {} => to_binary(&query_chain(deps, env)?),
        QueryMsg::QueryConstruct {} => to_binary(&query_construct(deps, env)?),
    }
}

pub fn query_result(_deps: DepsMut, _info: MessageInfo) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("method", "query_result"))
}

fn query_random(env: Env) -> StdResult<RandomResponse> {
    let mut rng = Rand32::new(env.block.height);
    Ok(RandomResponse {
        number: rng.rand_u32(),
    })
}

fn query_bool_binary_echo(msg: Option<Binary>) -> StdResult<RuleResponse<Option<Binary>>> {
    Ok((true, msg))
}

fn query_input_bool_binary(msg: Binary) -> StdResult<RuleResponse<Option<Binary>>> {
    let something: RandomResponse = from_binary(&msg)?;
    let data = Some(to_binary(&something)?);
    Ok((true, data))
}

// GOAL:
// Parse a generic query response, and inject input for the next query
fn query_chain(deps: Deps, env: Env) -> StdResult<QueryTestResponse> {
    // Get known format for first msg
    let msg1 = QueryMsg::GetRandom {};
    let res1: RandomResponse = deps
        .querier
        .query_wasm_smart(&env.contract.address, &msg1)?;

    // Query a bool with some data from previous
    let msg2 = QueryMsg::GetBoolBinary {
        msg: Some(to_binary(&res1)?),
    };
    let res2: RuleResponse<Option<Binary>> = deps
        .querier
        .query_wasm_smart(&env.contract.address, &msg2)?;

    // Utilize previous query for the input of this query
    // TODO: Setup binary msg, parse into something that contains { msg }, then assign the new binary response to it (if any)
    // let msg = QueryMsg::GetInputBoolBinary {
    //     msg: Some(to_binary(&res2)?),
    // };
    // let res: RuleResponse<Option<Binary>> =
    //     deps.querier.query_wasm_smart(&env.contract.address, &msg)?;

    // Format something to read results
    let data = format!("{:?}", res2);
    Ok(QueryTestResponse { data })
}

// create a smart query into binary
fn query_construct(_deps: Deps, _env: Env) -> StdResult<QueryTestResponse> {
    let input_binary = to_binary(&RandomResponse { number: 1235 })?;

    // create an injectable blank msg
    let json_msg = json!({
        "get_random": {
            "msg": "",
            "key": "value"
        }
    });
    // blank msg to binary
    let msg_binary = to_binary(&json_msg.to_string())?;

    // try to parse binary
    let msg_unbinary: String = from_binary(&msg_binary)?;
    // let msg_parsed: Value = serde_json::from_str(msg_unbinary);
    let msg_parse = serde_json::from_str(msg_unbinary.as_str());
    let mut msg_parsed: String = "".to_string();

    // Attempt to peel the onion, and fill with goodness
    if let Ok(msg_parse) = msg_parse {
        let parsed: Value = msg_parse;
        // travel n1 child keys
        if parsed.is_object() {
            for (_key, value) in parsed.as_object().unwrap().iter() {
                for (k, _v) in value.as_object().unwrap().iter() {
                    // check if this key has "msg"
                    if k == "msg" {
                        // then replace "msg" with "input_binary"
                        // TODO:
                        // parsed[key][k] = input_binary;
                        msg_parsed = input_binary.to_string();
                    }
                }
            }
        }
    }

    // Lastly, attempt to make the actual query!
    // let res1 = deps
    //     .querier
    //     .query_wasm_smart(&env.contract.address, &msg1)?;

    // Format something to read results
    // let data = format!("{:?}", res1);
    let data = format!("{:?} :: {:?}", msg_binary, msg_parsed);
    Ok(QueryTestResponse { data })
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
//     fn increment() {
//         let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Increment {};
//         let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // should increase counter by 1
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(18, value.count);
//     }
// }
