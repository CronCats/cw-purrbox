use crate::msgs::instantiate_msg::InstantiateMsg;
use crate::{BOOLEAN_ADDRESS, CONTRACT_NAME, CONTRACT_VERSION, CRONCAT_FACTORY_ADDRESS};
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use crate::errors::ContractError;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set the contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // With a contract this simple, we'll just set state directly.
    // Usually, though, you'll want to use cw-storage-plus.
    deps.storage.set(CRONCAT_FACTORY_ADDRESS, msg.croncat_factory_address.as_bytes());
    deps.storage.set(BOOLEAN_ADDRESS, msg.boolean_address.as_bytes());

    // Return a thumbs up
    Ok(Response::default())
}
