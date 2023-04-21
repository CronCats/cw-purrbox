use crate::errors::ContractError;
use crate::state::{CRONCAT_FACTORY_ADDRESS, MOCK_AUCTIONS};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

/// This "tick" method will clean out old auctions
/// (or any other regular maintenance or otherwise)
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // Contracts integrating CronCat should save the CronCat factory address,
    // which is not expected to change.

    // Load CronCat factory address from state
    let croncat_factory_addr = CRONCAT_FACTORY_ADDRESS.load(deps.storage)?;

    // Use CronCat helper to validate this invocation is coming from
    // a sanctioned version of a CronCat manager contract.

    // We can optionally change a few parameters to suit our needs.
    // Structure from crate croncat-integration-utils
    // CRONCAT HELPER
    let _extra_params = croncat_integration_utils::types::HandleIncomingTaskParams {
        // Whether to check if this transaction is in the same block
        // and transaction index as the last CronCat manager task sent.
        // If you expect an IBC delay or something asynchronous, disable by setting to true.
        disable_sync_check: false,
        // By default, we check that the contract receiving the task invocation
        // must be the owner of the that task. Put another way: someone else's
        // task isn't invoking our method. You can disable this by setting to true.
        disable_owner_check: false,
        // Perhaps the task owner isn't this contract, but you know the address.
        // You can add it here instead. None defaults to the contract being invoked.
        // If disable_owner_check is true, this is irrelevant.
        expected_owner: None,
    };

    // Call a CronCat integration helper function
    // We're not reading the variable below for this example, but you could.
    // CRONCAT HELPER
    let _task_info = croncat_integration_utils::handle_incoming_task::handle_incoming_task(
        &deps.querier,
        env.clone(),
        info,
        // Remember we load this from our contract's state.
        croncat_factory_addr,
        // We could use extra_params here, but this also means default.
        None,
    )?;

    // We've validated that this invocation is from a known CronCat task.

    // YOUR CUSTOM LOGIC

    // Now our tick function performs its objectives.
    // In our case, we'll remove old auctions.
    let mut auctions = MOCK_AUCTIONS.load(deps.storage)?;
    // Keep the auctions whose time is in the future
    auctions.retain(|auction| auction.end_time > env.block.time);
    // Save those bad boys
    MOCK_AUCTIONS.save(deps.storage, &auctions)?;

    Ok(Response::new().add_attribute("action", "tick"))
}
