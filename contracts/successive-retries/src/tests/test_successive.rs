use crate::msgs::instantiate_msg::InstantiateMsg;
use crate::msgs::query_msg::QueryMsg::FailedAttempts;
use crate::tests::contracts;
use cosmwasm_std::{coin, from_slice, Addr, Uint64};
use croncat_integration_testing::test_helpers::{
    add_seconds_to_block, increment_block_height, set_up_croncat_contracts, CronCatTestEnv,
};
use croncat_integration_testing::{AGENT, ALICE, DENOM};
use croncat_sdk_agents::msg::ExecuteMsg::RegisterAgent;
use croncat_sdk_manager::msg::ManagerExecuteMsg::ProxyCall;
use cw_multi_test::Executor;

const INITIAL_FEE: u128 = 555000;

#[test]
fn create_task_proxy_call() {
    let CronCatTestEnv {
        mut app,
        factory,
        manager,
        tasks,
        agents,
    } = set_up_croncat_contracts(None);

    let inst_msg = InstantiateMsg {
        croncat_factory_address: factory.to_string(),
        public_funding_address: "cosmos1yhqft6d2msmzpugdjtawsgdlwvgq3sam4fh4yj".to_string(),
        delay_in_minutes: 1,
        max_times_to_try: 3,
    };

    // Deploy this example contract
    let code_id = app.store_code(contracts::successive_retries());
    let example_address = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(ALICE),
            &inst_msg,
            &[coin(INITIAL_FEE, DENOM)],
            "create task handle tick 🤙",
            None,
        )
        .unwrap();

    // Remember it creates a CronCat task on instantiation

    // Register a CronCat agent who will execute the task
    // See: https://docs.cron.cat/docs/contracts-agents
    let agent_registration_res = app.execute_contract(
        Addr::unchecked(AGENT),
        agents,
        &RegisterAgent {
            payable_account_id: None,
        },
        &[],
    );

    assert!(agent_registration_res.is_ok());

    // Before executing the CronCat task, let's check how many
    // failed attempts there are
    let mut failed_attempts: u8 = app
        .wrap()
        .query_wasm_smart(example_address.clone(), &FailedAttempts {})
        .expect("Did not retrieve failed attempts properly");

    // Before the agent calls CronCat, fulfilling the task, we have three auctions
    assert_eq!(failed_attempts, 1u8);

    app.update_block(|block| add_seconds_to_block(block, 60));
    // Fast-forwarding blocks here is not strictly necessary, but let's have fun.
    app.update_block(|block| increment_block_height(block, Some(10)));

    // Run proxy_call from the agent
    let proxy_call_res = app.execute_contract(
        Addr::unchecked(AGENT),
        manager.clone(),
        &ProxyCall { task_hash: None },
        &[],
    );

    assert!(proxy_call_res.is_ok());

    // We should have two attempts because it instantiates to 1
    failed_attempts = app
        .wrap()
        .query_wasm_smart(example_address.clone(), &FailedAttempts {})
        .expect("Did not retrieve failed attempts properly");

    assert_eq!(failed_attempts, 2u8);

    // Now Alice sends double the funds, fast-forward, let the agent fulfill it.
    app.send_tokens(
        Addr::unchecked(ALICE),
        example_address.clone(),
        &[coin(1332000u128, DENOM)],
    )
    .expect("Alice should be able to donate bro");

    app.update_block(|block| add_seconds_to_block(block, 60));
    app.update_block(|block| increment_block_height(block, Some(10)));

    assert!(
        app.execute_contract(
            Addr::unchecked(AGENT),
            manager,
            &ProxyCall { task_hash: None },
            &[],
        )
        .is_ok(),
        "A subsequent proxy call busted"
    );

    // Check balance of public funding address
    let funding_address_raw = app
        .wrap()
        .query_wasm_raw(example_address, b"pfa".as_slice())
        .expect("Problem querying for public funding address")
        .unwrap();
    let funding_address: String = from_slice(funding_address_raw.as_slice()).unwrap();

    // Now check the balance of the funding address
    let public_funds_total = app
        .wrap()
        .query_balance(funding_address, DENOM)
        .expect("Issue querying balance of public funding");

    assert!(
        (public_funds_total.amount.u128() >= INITIAL_FEE * 2),
        "The public funds should have at least double the amount sent during instantiation"
    );

    // Confirm the task is removed by showing there are no tasks
    let tasks_total: Uint64 = app
        .wrap()
        .query_wasm_smart(tasks, &croncat_sdk_tasks::msg::TasksQueryMsg::TasksTotal {})
        .expect("Issue getting task total");
    assert_eq!(tasks_total.u64(), 0u64, "Should be no tasks");
}
