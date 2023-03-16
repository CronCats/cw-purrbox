use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, to_binary, SubMsg};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::CosmosMsg::Wasm;
use cosmwasm_std::WasmMsg::Execute;
use croncat_sdk_factory::msg::ContractMetadataResponse;
use croncat_sdk_tasks::msg::TasksExecuteMsg::CreateTask;
use crate::errors::ContractError;
use crate::{BOOLEAN_ADDRESS, CRONCAT_FACTORY_ADDRESS, REPLY_CRONCAT_TASK_CREATION};
use croncat_sdk_tasks::types::{Action, Interval, TaskRequest};
use croncat_sdk_factory::msg::FactoryQueryMsg::LatestContract;
use crate::utils::get_addr_from_state;

// Let's say we don't have a package/crate but know the structure
#[cw_serde]
pub enum BooleanContractExecuteMsg {
    Toggle {},
}

pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // To create a CronCat task you will need to provide funds. All funds sent
    // to this method will be used for task creation
    // Because we cannot detect detailed error information from replies
    // (See CosmWasm Discord: https://discord.com/channels/737637324434833438/737643344712171600/1040920787512725574)
    // We'll add a check here to ensure they've attached funds
    if info.funds.is_empty() {
        return Err(ContractError::NoFundsAttached {})
    }

    // This will validate the addresses and throw an error (see "?") if invalid
    let croncat_factory_address: String = get_addr_from_state(&deps, CRONCAT_FACTORY_ADDRESS);
    let boolean_address: String = get_addr_from_state(&deps, BOOLEAN_ADDRESS);

    // We'll call the CronCat Factory contract and ask for the address of the "tasks" contract
    let tasks_name: String = String::from("tasks");

    // Ask the CronCat Factory contract what the latest task contract address is
    // then we'll call `create_task` on the provided Task contract
    let query_factory_msg = LatestContract {
        contract_name: tasks_name.clone(),
    };
    let latest_contract_res: ContractMetadataResponse = deps.querier.query_wasm_smart(&croncat_factory_address, &query_factory_msg)?;

    // Check validity of result
    if latest_contract_res.metadata.is_none() {
        return Err(ContractError::CustomError {
            code: "NO_SUCH_CONTRACT_NAME_ON_FACTORY".to_string(),
            msg: format!("Did not find contract '{}' on factory contract {}", tasks_name, croncat_factory_address),
        })
    }

    let tasks_address = latest_contract_res.metadata.unwrap().contract_addr;

    let croncat_task = TaskRequest {
        interval: Interval::Block(1),
        boundary: None,
        stop_on_fail: true,
        actions: vec![Action {
            msg: Wasm(Execute {
                contract_addr: boolean_address.clone(),
                msg: to_binary(
                    &BooleanContractExecuteMsg::Toggle {},
                )?,
                funds: vec![],
            }),
            gas_limit: Some(550_000), // fine tune gas here
        }],
        queries: None,
        transforms: None,
        cw20: None,
    };

    let create_task_msg = Wasm(Execute {
        contract_addr: String::from(tasks_address.clone()),
        msg: to_binary(
            &CreateTask {
                task: Box::new(croncat_task),
            },
        )?,
        funds: info.funds,
    });

    let sub_message = SubMsg::reply_on_error(create_task_msg, REPLY_CRONCAT_TASK_CREATION);

    Ok(Response::new()
        .add_attribute("croncat_factory_address", croncat_factory_address)
        .add_attribute("boolean_address", boolean_address)
        .add_attribute("tasks_address", tasks_address)
        .add_submessage(sub_message)
    )
}
