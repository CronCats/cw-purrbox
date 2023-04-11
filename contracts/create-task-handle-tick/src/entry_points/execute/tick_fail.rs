use crate::errors::ContractError;
use crate::state::CRONCAT_FACTORY_ADDRESS;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use croncat_sdk_factory::msg::ContractMetadataResponse;
use croncat_sdk_factory::msg::FactoryQueryMsg::LatestContract;
use croncat_sdk_tasks::types::TaskExecutionInfo;

#[cw_serde]
pub enum NewManagerMethods {
    LatestTaskExecution {},
}

pub fn execute(deps: DepsMut, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    // This method will clean out old auctions (or whatever you want)

    // Ask CronCat Factory if the sender is indeed a version of the Manager contract
    // TODO: need to add a query method on factory that takes a contract address and tells you if it's one of ours

    let croncat_factory_address = CRONCAT_FACTORY_ADDRESS.load(deps.storage)?;

    let query_factory_msg = LatestContract {
        contract_name: String::from("manager"),
    };
    let latest_contract_res: ContractMetadataResponse = deps
        .querier
        .query_wasm_smart(&croncat_factory_address, &query_factory_msg)?;

    // Check validity of result
    if latest_contract_res.metadata.is_none() {
        return Err(ContractError::CustomError {
            code: "NO_SUCH_CONTRACT_NAME_ON_FACTORY".to_string(),
            msg: format!(
                "Did not find manager contract on factory contract {}",
                croncat_factory_address
            ),
        });
    }

    let manager_address = latest_contract_res.metadata.unwrap().contract_addr;

    let latest_task_execution_res: TaskExecutionInfo = deps.querier.query_wasm_smart(
        manager_address.clone(),
        &NewManagerMethods::LatestTaskExecution {},
    )?;

    // Turn it into nice, clean JSON and return that
    let latest_task_execution = serde_json::to_string(&latest_task_execution_res.clone()).unwrap();
    println!("aloha latest_task_execution: {:?}", latest_task_execution);

    Err(ContractError::CustomError {
        code: "INTENTIONAL_FAILURE".to_string(),
        msg: format!(
            "{} — {}",
            "Let's check how the state of the Manager looks".to_string(),
            latest_task_execution
        ),
    })
}
