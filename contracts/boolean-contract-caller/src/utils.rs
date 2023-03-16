use cosmwasm_std::DepsMut;

// We already validated the addresses before saving them,
// hence the reckless use of unwrap()
pub fn get_addr_from_state(deps: &DepsMut, key: &[u8]) -> String {
  let factory_address_from_state = deps.storage.get(key);
  String::from_utf8(factory_address_from_state.unwrap()).unwrap()
}