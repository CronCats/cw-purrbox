use crate::errors::ContractError;
use crate::msgs::instantiate_msg::InstantiateMsg;
use crate::state::{BOOLEAN_ADDRESS, CRONCAT_FACTORY_ADDRESS};
use crate::{CONTRACT_NAME, CONTRACT_VERSION};
use cosmwasm_std::{entry_point, Addr};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set the contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Validate all addresses received
    deps.api
        .addr_validate(msg.croncat_factory_address.as_str())?;
    deps.api.addr_validate(msg.boolean_address.as_str())?;

    CRONCAT_FACTORY_ADDRESS.save(deps.storage, &Addr::unchecked(msg.croncat_factory_address))?;
    BOOLEAN_ADDRESS.save(deps.storage, &Addr::unchecked(msg.boolean_address))?;

    // Return a thumbs up
    Ok(Response::default())
}
