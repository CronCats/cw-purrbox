use crate::errors::ContractError;
use crate::msgs::instantiate_msg::InstantiateMsg;
use crate::state::{Config, CONFIG};
use crate::{CONTRACT_NAME, CONTRACT_VERSION};
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

/// Instantiate entry point
/// See the instantiate message and fields in [InstantiateMsg](InstantiateMsg)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _first_param: Option<InstantiateMsg>,
) -> Result<Response, ContractError> {
    // Set the contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Need to explicitly set the state storage, which is just is_true basically
    CONFIG.save(deps.storage, &Config { is_true: false })?;
    Ok(Response::default())
}
