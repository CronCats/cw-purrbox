use cosmwasm_std::Empty;
use cw_multi_test::{Contract, ContractWrapper};

pub(crate) fn boolean_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw_boolean_contract::entry_points::execute::execute,
        cw_boolean_contract::entry_points::instantiate::instantiate,
        cw_boolean_contract::entry_points::query::query,
    );
    Box::new(contract)
}

pub(crate) fn boolean_contract_caller() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::entry_points::execute::execute,
        crate::entry_points::instantiate::instantiate,
        crate::entry_points::query::query,
    )
    .with_reply(crate::entry_points::reply::reply);
    Box::new(contract)
}
