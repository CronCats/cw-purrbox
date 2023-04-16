use crate::errors::ContractError;
use crate::msgs::execute_msg::ExecuteMsg;
use crate::state::CRONCAT_FACTORY_ADDRESS;
use crate::REPLY_CRONCAT_TASK_CREATION;
use cosmwasm_std::CosmosMsg::Wasm;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response, SubMsg};
use croncat_sdk_factory::msg::ContractMetadataResponse;
use croncat_sdk_factory::msg::FactoryQueryMsg::LatestContract;
use croncat_sdk_tasks::msg::TasksExecuteMsg::CreateTask;
use croncat_sdk_tasks::types::{Action, Interval, TaskRequest};

/// This method intentionally fails because it doesn't
/// attach funds to pay the agent and so on.
/// This demonstrates how the Reply message is handled.
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // The code below is copy/pasted from make_croncat_tick_task
    // and is deliberately duplicate.
    // If you're using this example as a template, feel free to remove this and tick_fail
    let croncat_factory_address = CRONCAT_FACTORY_ADDRESS.load(deps.storage)?;

    // We'll call the CronCat Factory contract and ask for the address of the "tasks" contract
    let tasks_name: String = String::from("tasks");

    // Ask the CronCat Factory contract what the latest task contract address is
    // then we'll call `create_task` on the provided Task contract
    let query_factory_msg = LatestContract {
        contract_name: tasks_name.clone(),
    };
    let latest_contract_res: ContractMetadataResponse = deps
        .querier
        .query_wasm_smart(&croncat_factory_address, &query_factory_msg)?;

    // Check validity of result
    if latest_contract_res.metadata.is_none() {
        return Err(ContractError::CustomError {
            code: "NO_SUCH_CONTRACT_NAME_ON_FACTORY".to_string(),
            msg: format!(
                "Did not find contract '{}' on factory contract {}",
                tasks_name, croncat_factory_address
            ),
        });
    }

    let tasks_address = latest_contract_res.metadata.unwrap().contract_addr;

    let croncat_task = TaskRequest {
        interval: Interval::Block(1),
        boundary: None,
        stop_on_fail: false, // So we can reproduce the error repeatedly
        actions: vec![Action {
            msg: Wasm(Execute {
                // Call "myself" at the tick method
                contract_addr: env.contract.address.to_string(),
                msg: to_binary(&ExecuteMsg::TickFail {})?,
                funds: vec![],
            }),
            gas_limit: Some(550_000), // can fine tune gas here
        }],
        queries: None,
        transforms: None,
        cw20: None,
    };

    let create_task_msg = Wasm(Execute {
        contract_addr: String::from(tasks_address.clone()),
        msg: to_binary(&CreateTask {
            task: Box::new(croncat_task),
        })?,
        funds: info.funds,
    });

    let sub_message = SubMsg::reply_always(create_task_msg, REPLY_CRONCAT_TASK_CREATION);

    Ok(Response::new()
        .add_attribute("action", "make_croncat_tick_task")
        .add_submessage(sub_message))
}
