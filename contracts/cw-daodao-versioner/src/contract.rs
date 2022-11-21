#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg,
};
use cw2::set_contract_version;
use cw_croncat_core::msg::TaskRequest;
use cw_croncat_core::types::{Action, Interval};

use crate::daodao::create_daodao_proposal;
//use crate::daodao::create_daodao_proposal;
use crate::error::ContractError;
use crate::msg::dao_registry::query::*;
use crate::msg::dao_registry::state::Registration;
use crate::msg::*;
use crate::state::{CRONCAT_ADDR, REGISTRAR_ADDR, VERSION_MAP};
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-daodao-versioner";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let registrar_addr = deps.api.addr_validate(&_msg.registrar_addr)?;
    let croncat_addr = deps.api.addr_validate(&_msg.croncat_addr)?;

    REGISTRAR_ADDR.save(deps.storage, &registrar_addr)?;
    CRONCAT_ADDR.save(deps.storage, &croncat_addr)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::QueryResult {} => query_result(deps, info),
        ExecuteMsg::CreateVersioner {
            daodao_addr,
            name,
            chain_id,
        } => create_versioner(deps, env, daodao_addr, name, chain_id, info.funds),
        ExecuteMsg::RemoveVersioner { name, chain_id } => remove_versioner(deps, name, chain_id),
        ExecuteMsg::UpdateVersioner {
            daodao_addr,
            name,
            chain_id,
        } => update_versioner(deps, env, info, daodao_addr, name, chain_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VerifyNewVersionAvailable { name, chain_id } => {
            to_binary(&query_new_version_available(deps, name, chain_id)?)
        }
    }
}

pub fn query_result(_deps: DepsMut, _info: MessageInfo) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("method", "query_result"))
}

fn _query_registrations(
    deps: Deps,
    registrar_addr: String,
    name: String,
    chain_id: String,
) -> StdResult<ListRegistrationsResponse> {
    let registrar_address = deps.api.addr_validate(&registrar_addr)?;
    let res: ListRegistrationsResponse = deps.querier.query_wasm_smart(
        registrar_address,
        &RegistryQueryMsg::ListRegistrations { name, chain_id },
    )?;
    Ok(res)
}
fn _query_code_id_info(
    deps: Deps,
    registrar_addr: String,
    chain_id: String,
    code_id: u64,
) -> StdResult<GetRegistrationResponse> {
    let registrar_address = deps.api.addr_validate(&registrar_addr)?;
    let res: GetRegistrationResponse = deps.querier.query_wasm_smart(
        registrar_address,
        &RegistryQueryMsg::GetCodeIdInfo { chain_id, code_id },
    )?;
    Ok(res)
}
fn query_registration(
    deps: Deps,
    registrar_addr: String,
    contract_name: String,
    chain_id: String,
    version: Option<String>,
) -> StdResult<GetRegistrationResponse> {
    let registrar_address = deps.api.addr_validate(&registrar_addr)?;
    let res: GetRegistrationResponse = deps.querier.query_wasm_smart(
        registrar_address,
        &RegistryQueryMsg::GetRegistration {
            name: contract_name,
            chain_id,
            version,
        },
    )?;
    Ok(res)
}
fn create_versioner(
    deps: DepsMut,
    env: Env,
    daodao_addr: String,
    name: String,
    chain_id: String,
    funds: Vec<Coin>,
) -> Result<Response, ContractError> {
    if VERSION_MAP
        .may_load(deps.storage, (&name, &chain_id))?
        .is_some()
    {
        return Err(ContractError::ContractAlreadyRegistered(name, chain_id));
    }
    let registrar_addr = REGISTRAR_ADDR.load(deps.storage)?;
    let registration = query_registration(
        deps.as_ref(),
        registrar_addr.to_string(),
        name.clone(),
        chain_id.clone(),
        None,
    )?
    .registration;
    VERSION_MAP.save(
        deps.storage,
        (&registration.contract_name, &chain_id),
        &registration.version,
    )?;
    //create a croncat task for version check
    let resp = create_versioner_cron_task(
        deps,
        env,
        daodao_addr,
        name.clone(),
        chain_id.clone(),
        funds,
    )
    .unwrap();

    Ok(resp
        .add_attribute("action", "create_versioner")
        .add_attribute("contract_name", name)
        .add_attribute("chain_id", chain_id)
        .add_attribute("current_version", &registration.version))
}

fn remove_versioner(
    deps: DepsMut,
    name: String,
    chain_id: String,
) -> Result<Response, ContractError> {
    if VERSION_MAP
        .may_load(deps.storage, (&name, &chain_id))?
        .is_none()
    {
        return Err(ContractError::ContractNotRegistered(name, chain_id));
    }

    VERSION_MAP.remove(deps.storage, (&name, &chain_id));

    Ok(Response::new()
        .add_attribute("action", "remove_contract_versioner")
        .add_attribute("contract_name", name)
        .add_attribute("chain_id", chain_id))
}

fn is_new_version_available(deps: Deps, name: String, chain_id: String) -> (bool, Registration) {
    let registrar_addr = REGISTRAR_ADDR.load(deps.storage).unwrap();
    let registration = query_registration(
        deps,
        registrar_addr.to_string(),
        name.clone(),
        chain_id.clone(),
        None,
    )
    .unwrap()
    .registration;

    let current_version = VERSION_MAP.may_load(deps.storage, (&name, &chain_id));
    (
        registration.version > current_version.unwrap().unwrap_or_default(),
        registration,
    )
}

fn query_new_version_available(deps: Deps, name: String, chain_id: String) -> StdResult<bool> {
    Ok(is_new_version_available(deps, name, chain_id).0)
}

fn update_versioner(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    daodao_addr: String,
    name: String,
    chain_id: String,
) -> Result<Response, ContractError> {
    let verify = is_new_version_available(deps.as_ref(), name.clone(), chain_id.clone());
    //If new version is available, create proposal
    if verify.0 {
        let result = create_daodao_proposal(daodao_addr, name.clone(), chain_id.clone(), None);
        VERSION_MAP.save(deps.storage, (&name, &chain_id), &verify.1.version)?;
        return result;
    }
    Ok(Response::new()
        .add_attribute("action", "update_versioner")
        .add_attribute("up_to_date", "yes")
        .add_attribute("version", verify.1.version))
}
fn create_versioner_cron_task(
    deps: DepsMut,
    env: Env,
    daodao_addr: String,
    name: String,
    chain_id: String,
    funds: Vec<Coin>,
) -> Result<Response, ContractError> {
    let croncat_addr = CRONCAT_ADDR.load(deps.storage)?;
    //let cron_name = format!("{name}{chain_id}");
    let action = Action {
        msg: WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&ExecuteMsg::UpdateVersioner {
                daodao_addr,
                name,
                chain_id,
            })?,
            funds: vec![],
        }
        .into(),
        gas_limit: Some(900_000),
    };

    let task_request = TaskRequest {
        interval: Interval::Block(10),
        boundary: None,
        stop_on_fail: false,
        actions: vec![action],
        rules: None,
        cw20_coins: vec![],
    };

    let cronmsg = cw_croncat_core::msg::ExecuteMsg::CreateTask { task: task_request };

    let msg = WasmMsg::Execute {
        contract_addr: croncat_addr.to_string(),
        msg: to_binary(&cronmsg)?,
        funds,
    };

    Ok(Response::new()
        .add_attribute("action", "create_versioner_cron_task")
        .add_message(msg))
}
