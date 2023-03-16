use crate::errors::ContractError;
use crate::state::{Config, CONFIG};
use cosmwasm_std::{AllDelegationsResponse, Coin, Delegation, DepsMut, Env, FullDelegation, MessageInfo, Response, StakingQuery, to_binary};
use cosmwasm_std::CosmosMsg::Staking;

pub fn execute(deps: DepsMut, _env: Env, _info: MessageInfo, delegator: String, validator: String) -> Result<Response, ContractError> {
  let full_delegation: Option<FullDelegation> = deps.querier.query_delegation(delegator, validator)?;
  // let full_delegation: Vec<Delegation> = deps.querier.query_all_delegations(delegator)?;

  // let readable = serde_json::to_string(&full_delegation).unwrap();
  let amount: Coin = if let Some(delegation) = full_delegation {
    delegation.amount
  } else { Coin::default() };
  let readable = serde_json::to_string(&amount).unwrap();

  // StakingQuery::AllDelegations {}
  //   AllDelegationsResponse
  // CONFIG.save(deps.storage, &toggle_boolean)?;
  Ok(Response::new()
    .add_attribute("action", "check_stake")
    .set_data(to_binary(&readable).unwrap())
    .add_attribute("delegation_amount", readable)
    // .set_data(to_binary(&full_delegation)?
  )
}
