use crate::errors::ContractError;
use crate::state::CRONCAT_FACTORY_ADDRESS;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use croncat_sdk_factory::msg::EntryResponse;
use croncat_sdk_factory::msg::FactoryQueryMsg::LatestContracts;

pub fn execute(deps: DepsMut, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let croncat_factory_address = CRONCAT_FACTORY_ADDRESS.load(deps.storage)?;

    // Query the CronCat Factory asking for the latest version of all contracts
    let factory_latest_contracts = LatestContracts {};
    let latest_contracts_res: Vec<EntryResponse> = deps
        .querier
        .query_wasm_smart(&croncat_factory_address, &factory_latest_contracts)?;

    // Turn it into nice, clean JSON and return that
    let latest_contracts = serde_json::to_string(&latest_contracts_res).unwrap();

    Ok(Response::new().add_attribute("latest_contracts", latest_contracts))
}
