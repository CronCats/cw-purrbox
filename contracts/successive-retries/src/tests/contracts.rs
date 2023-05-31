use cosmwasm_std::Empty;
use cw_multi_test::{Contract, ContractWrapper};

// We create this and use the croncat-integration-utils contracts
pub(crate) fn successive_retries() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::entry_points::execute::execute,
        crate::entry_points::instantiate::instantiate,
        crate::entry_points::query::query,
    );
    Box::new(contract)
}
