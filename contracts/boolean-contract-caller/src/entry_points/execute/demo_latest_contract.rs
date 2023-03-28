use crate::errors::ContractError;
use crate::state::CRONCAT_FACTORY_ADDRESS;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use croncat_sdk_factory::msg::ContractMetadataResponse;
use croncat_sdk_factory::msg::FactoryQueryMsg::LatestContract;

pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    contract_name: String,
) -> Result<Response, ContractError> {
    let croncat_factory_address = CRONCAT_FACTORY_ADDRESS.load(deps.storage)?;

    // Ask the CronCat Factory contract what the latest task contract address is
    // then we'll call `create_task` on the provided Task contract
    let factory_latest_contract = LatestContract {
        contract_name: contract_name.clone(),
    };
    let latest_contract_res: ContractMetadataResponse = deps
        .querier
        .query_wasm_smart(&croncat_factory_address, &factory_latest_contract)?;

    // Check validity of result
    if latest_contract_res.metadata.is_none() {
        return Err(ContractError::CustomError {
            code: "NO_SUCH_CONTRACT_NAME_ON_FACTORY".to_string(),
            msg: format!(
                "Did not find contract '{}' on factory contract {}",
                contract_name, croncat_factory_address
            ),
        });
    }

    let manager_address = latest_contract_res.metadata.unwrap().contract_addr;

    Ok(Response::new().add_attribute("manager_address", manager_address))
}
