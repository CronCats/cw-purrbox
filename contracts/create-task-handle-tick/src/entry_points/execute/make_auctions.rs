use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::errors::ContractError;
use crate::state::MOCK_AUCTIONS;
use crate::utils::get_mock_auctions;

pub fn execute(deps: DepsMut, env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
  // Anyone can call this method, so no checks here
  let new_mock_auctions = get_mock_auctions(&env);

  // Load it
  let mut current_auctions = MOCK_AUCTIONS.load(deps.storage)?;
  // Add the new ones to the end
  current_auctions.extend(new_mock_auctions);
  // Save it
  MOCK_AUCTIONS.save(deps.storage, &current_auctions)?;

  Ok(Response::default())
}