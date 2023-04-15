use crate::state::Auction;
use crate::TEN_SECONDS_IN_NANOS;
use cosmwasm_std::{DepsMut, Env, Timestamp};

// We already validated the addresses before saving them,
// hence the reckless use of unwrap()
pub fn get_addr_from_state(deps: &DepsMut, key: &[u8]) -> String {
    let factory_address_from_state = deps.storage.get(key);
    String::from_utf8(factory_address_from_state.unwrap()).unwrap()
}

pub fn get_mock_auctions(env: &Env) -> Vec<Auction> {
    let mut auctions = vec![];
    let phonetic_alphabet = ["alpha", "bravo", "charlie"];

    for (idx, name) in phonetic_alphabet.iter().enumerate() {
        auctions.push(Auction {
            // Each a minute further in the future
            end_time: Timestamp::from_nanos(
                env.block.time.nanos() + TEN_SECONDS_IN_NANOS * (idx as u64 + 1),
            ),
            title: format!("{} is for sale!", name),
        })
    }

    auctions
}
