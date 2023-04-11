use std::vec;
use crate::errors::ContractError;
use crate::state::{CRONCAT_FACTORY_ADDRESS, MOCK_AUCTIONS};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, DepsMut, Env, from_slice, MessageInfo, QueryRequest, Response, StdError, StdResult, to_binary, Uint64, WasmQuery};
use croncat_sdk_factory::msg::EntryResponse;
use croncat_sdk_factory::state::CONTRACT_ADDRS;
use croncat_sdk_manager::types::LAST_TASK_EXECUTION_INFO_KEY;
use croncat_sdk_tasks::types::TaskExecutionInfo;
use cosmwasm_storage::{to_length_prefixed, to_length_prefixed_nested};
use croncat_sdk_factory::msg::Config;

#[cw_serde]
pub enum NewManagerMethods {
    LatestTaskExecution {},
}

#[cw_serde]
pub struct LatestTaskExecutionInfo {
    pub block_height: Uint64,
    pub owner_addr: Addr,
    pub task_hash: String,
}

#[cw_serde]
struct StorageKey<'a> {
    manager: &'a str,
    version: [u8; 2],
}

pub fn execute(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // This method will clean out old auctions (or whatever you want)

    // Ask CronCat Factory if the sender is indeed a version of the Manager contract
    // TODO: need to add a query method on factory that takes a contract address and tells you if it's one of ours

    let croncat_factory_address = CRONCAT_FACTORY_ADDRESS.load(deps.storage)?;

    // We want to confirm this comes from a sanctioned, CronCat manager
    // contract, which we'll do when we query the factory a bit later

    let sender = info.sender;

    let latest_task_execution_res = deps.querier.query_wasm_raw(sender.clone(), LAST_TASK_EXECUTION_INFO_KEY.as_bytes().to_vec())?;

    if latest_task_execution_res.is_none() {
        return Err(ContractError::CustomError {
            code: "COULD_NOT_RETRIEVE_LATEST_TASK_INFO".to_string(),
            msg: "Expected to be able to retrieve state from last task execution info key on the CronCat manager".to_string(),
        })
    }

    let latest_task_execution_cast_res = serde_json_wasm::from_slice(latest_task_execution_res.unwrap().as_slice());

    if latest_task_execution_cast_res.is_err() {
        return Err(ContractError::CustomError {
            code: "CANNOT_CAST_LATEST_TASK_INFO".to_string(),
            msg: "Problem deserializing latest task info".to_string(),
        })
    }

    let latest_task_execution: TaskExecutionInfo = latest_task_execution_cast_res.unwrap();

    let versions = latest_task_execution.version.split('.').map(|v| -> u8 {
        v.parse().unwrap()
    }).collect::<Vec<u8>>();
    println!("aloha versions {:?}", versions);

    let mut state_key = to_length_prefixed_nested(&["contract_addrs".as_bytes(), "manager".as_bytes()]);
    state_key.extend_from_slice(versions.as_slice());
    println!("aloha state_key \n{:?}", state_key);

    let sanctioned_manager_res = deps.querier.query_wasm_raw(
        croncat_factory_address.to_string(),
        Binary::from(state_key),
    )?;

    if sanctioned_manager_res.is_none() {
        return Err(ContractError::CustomError {
            code: "NO_MANAGER_ADDR_RESPONSE".to_string(),
            msg: "Could not get the manager address from factory given version".to_string(),
        })
    }

    let sanctioned_manager_address = sanctioned_manager_res.clone().unwrap();

    println!("aloha sanctioned_manager_address \n{:?}", sanctioned_manager_address);
    println!("aloha info sender \n{:?}", sender.clone().as_bytes().clone().to_vec());

    let quoted_sender = format!(r#""{}""#, sender);
    let quoted_sender_bytes = quoted_sender.as_bytes();
    println!("aloha quoted_sender \n{:?}", quoted_sender_bytes.clone());

    // If the sender and the sanctioned manager address differ,
    // then this isn't being called by CronCat
    if sanctioned_manager_address != quoted_sender_bytes {
        return Err(ContractError::CustomError {
            code: "NOT_CALLED_FROM_SANCTIONED_MANAGER".to_string(),
            msg: "Sender is not the manager contract".to_string(),
        })
    }

    // Require that this is both in the same block…
    let is_same_block_bool = env.block.height == latest_task_execution.block_height;
    // …and the same transaction index, meaning we're in the
    // middle of a cross-contract call from a sanctioned
    // CronCat manager contract.
    let is_same_tx_id_bool = env.transaction == latest_task_execution.tx_info;

    if !is_same_block_bool || !is_same_tx_id_bool {
        return Err(ContractError::CustomError {
            code: "NOT_SAME_BLOCK_OR_TX_INDEX".to_string(),
            msg: "The call to tick did not occur at both the same block and same transaction index".to_string(),
        })
    }

    println!("all g");

    // Now our tick function does whatever is helpful,
    // perhaps for regular maintenance.
    // In our case, we'll remove old auctions
    // (mock auctions whose expiration time is before "now")

    let mut auctions = MOCK_AUCTIONS.load(deps.storage)?;
    // Keep the auctions whose time is in the future
    auctions.retain(|auction| {
        println!("aloha auction.end_time \n\t{:?}", auction.end_time);
        println!("aloha env.block.time \n\t{:?}", env.block.time);
        auction.end_time > env.block.time
    });
    MOCK_AUCTIONS.save(deps.storage, &auctions)?;

    Ok(Response::new()
        .add_attribute("action", "tick")
        .add_attribute("is_same_tx_id_bool", is_same_tx_id_bool.to_string())
        .add_attribute("called_in_same_block", is_same_block_bool.to_string())
    )
}
