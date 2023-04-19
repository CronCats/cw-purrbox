use crate::errors::ContractError;
use crate::msgs::execute_msg::ExecuteMsg::Tick;
use crate::msgs::query_msg::QueryMsg::Auctions;
use crate::state::Auction;
use crate::tests::contracts;
use cosmwasm_std::CosmosMsg::Wasm;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{coin, coins, from_binary, to_binary, Addr, BankMsg, TransactionInfo};
use croncat_integration_testing::test_helpers::{
    add_seconds_to_block, default_app, increment_block_height, set_up_croncat_contracts,
    CronCatTestEnv,
};
use croncat_integration_testing::{AGENT, ALICE, BOB, DENOM, VERSION};
use croncat_sdk_agents::msg::ExecuteMsg::RegisterAgent;
use croncat_sdk_core::types::GasPrice;
use croncat_sdk_manager::msg::ManagerExecuteMsg::ProxyCall;
use croncat_sdk_tasks::types::{
    Action, AmountForOneTask, Boundary, BoundaryHeight, Interval, Task, TaskExecutionInfo,
    TaskRequest,
};
use cw_multi_test::Executor;

/// This test demonstrates how you might set up tests
/// that include CronCat in your dApp's workflow
#[test]
fn task_creation_directly() {
    let CronCatTestEnv {
        mut app,
        factory: _,
        manager: _,
        tasks,
        agents: _,
    } = set_up_croncat_contracts(None);

    // Create a task
    let action = Action {
        msg: BankMsg::Send {
            to_address: Addr::unchecked(BOB).to_string(),
            amount: coins(10, DENOM),
        }
        .into(),
        gas_limit: Some(100_000),
    };

    let task = TaskRequest {
        interval: Interval::Once,
        boundary: Some(Boundary::Height(BoundaryHeight {
            start: Some((app.block_info().height).into()),
            end: Some((app.block_info().height + 10).into()),
        })),
        stop_on_fail: false,
        actions: vec![action],
        queries: None,
        transforms: None,
        cw20: None,
    };

    let create_task_res = app
        .execute_contract(
            Addr::unchecked(ALICE),
            tasks.clone(),
            &croncat_sdk_tasks::msg::TasksExecuteMsg::CreateTask {
                task: Box::new(task.clone()),
            },
            &coins(300_000, DENOM),
        )
        .expect("Couldn't create first task");

    let task_info: TaskExecutionInfo = from_binary(&create_task_res.data.unwrap()).unwrap();

    let expected_task_info = TaskExecutionInfo {
        block_height: app.block_info().height,
        tx_info: Some(TransactionInfo { index: 0 }),
        task_hash: "atom:1e5e835598b0c3cc0a8c583611f826c1a1fcb442034683466b11ac6fe29".to_string(),
        owner_addr: Addr::unchecked(ALICE),
        amount_for_one_task: AmountForOneTask {
            cw20: None,
            coin: [Some(coin(10, DENOM)), None],
            gas: 400000,
            agent_fee: 5,
            treasury_fee: 5,
            gas_price: GasPrice {
                numerator: 4,
                denominator: 100,
                gas_adjustment_numerator: 150,
            },
        },
        version: "0.1".to_string(),
    };

    assert_eq!(task_info, expected_task_info);

    // Now, for demonstration purposes, we'll make this human-readable and print it
    let task_info_json =
        serde_json::to_string_pretty(&task_info).expect("Could not turn task info into JSON");
    // remember, to see this you gotta:
    // cargo test -- --nocapture
    println!(
        "Directly call CronCat Tasks contract\n---\ntask_info: {}\n---",
        task_info_json
    );
}

#[test]
fn task_creation_from_caller() {
    let CronCatTestEnv {
        mut app,
        factory,
        manager: _,
        tasks,
        agents: _,
    } = set_up_croncat_contracts(None);

    // Deploy this example contract
    let code_id = app.store_code(contracts::create_task_handle_tick());
    let example_address = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(ALICE),
            &crate::msgs::instantiate_msg::InstantiateMsg {
                croncat_factory_address: factory.to_string(),
            },
            &[],
            "create task handle tick 🤙",
            None,
        )
        .unwrap();

    let create_task_handle_tick_res = app.execute_contract(
        Addr::unchecked(ALICE),
        example_address.clone(),
        &crate::msgs::execute_msg::ExecuteMsg::MakeCroncatTickTask {},
        coins(1_000_000, DENOM).as_slice(),
    );

    assert!(create_task_handle_tick_res.is_ok());

    let create_task_handle_tick = create_task_handle_tick_res.unwrap();

    let task_info_binary = create_task_handle_tick
        .data
        .expect("Should be able to extract data from task creation");
    let task_info_slice = task_info_binary.0.as_slice();
    let created_task_info: TaskExecutionInfo = serde_json::from_slice(task_info_slice).unwrap();

    // Determine expected task hash by querying `task_hash`
    // Basically we feed task details to a query and can get the
    // deterministic task hash. Task hashes can never clash, and
    // an attempt at creating a task with the same hash will fail.
    let expected_task_hash: String = app
        .wrap()
        .query_wasm_smart(
            tasks,
            &croncat_sdk_tasks::msg::TasksQueryMsg::TaskHash {
                task: Box::new(Task {
                    owner_addr: example_address.clone(),
                    interval: Interval::Block(1),
                    boundary: Boundary::Height(BoundaryHeight {
                        start: None,
                        end: None,
                    }),
                    stop_on_fail: true,
                    actions: vec![Action {
                        msg: Wasm(Execute {
                            contract_addr: example_address.clone().to_string(),
                            msg: to_binary(&Tick {})
                                .expect("Could not turn the tick task into binary"),
                            funds: vec![],
                        }),
                        gas_limit: Some(550_000), // can fine tune gas here
                    }],
                    queries: vec![],
                    transforms: vec![],
                    version: VERSION.to_string(),
                    amount_for_one_task: Default::default(),
                }),
            },
        )
        .expect("Issue determining task hash");

    assert_eq!(
        created_task_info.task_hash, expected_task_hash,
        "Task hashes differ"
    );
    assert_eq!(
        created_task_info.owner_addr, example_address,
        "Unexpected owner"
    );
    assert_eq!(
        created_task_info.block_height,
        app.block_info().height,
        "Differing block heights"
    );
    // Ideally we'd like to check the transaction index as well, but cw-multi-test uses 0 for now.
}

#[test]
fn tick_directly() {
    let CronCatTestEnv {
        mut app,
        factory,
        manager: _,
        tasks: _,
        agents: _,
    } = set_up_croncat_contracts(None);

    // Deploy this example contract
    let code_id = app.store_code(contracts::create_task_handle_tick());
    let example_address = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(ALICE),
            &crate::msgs::instantiate_msg::InstantiateMsg {
                croncat_factory_address: factory.to_string(),
            },
            &[],
            "create task handle tick 🤙",
            None,
        )
        .unwrap();

    let create_tick_res = app.execute_contract(
        Addr::unchecked(ALICE),
        example_address.clone(),
        &Tick {},
        &[],
    );

    // We expect this to fail because `tick` enforces that it must:
    // - be called by a known CronCat manager
    // - be at the same block height and transaction index
    // - be coming from a task from a known owner
    let result_error: ContractError = create_tick_res.unwrap_err().downcast().unwrap();

    let expected_error = ContractError::CronCatError {
        // We'll show the full path here, demonstrating
        // this is from the utilities, all wrapped up.
        // CRONCAT HELPER
        err: croncat_integration_utils::error::CronCatContractError::LatestTaskInfoFailed {
            manager_addr: Addr::unchecked(ALICE),
        },
    };

    assert_eq!(result_error, expected_error);
}

#[test]
fn create_task_proxy_call() {
    let CronCatTestEnv {
        mut app,
        factory,
        manager,
        tasks: _,
        agents,
    } = set_up_croncat_contracts(None);

    // Deploy this example contract
    let code_id = app.store_code(contracts::create_task_handle_tick());
    let example_address = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(ALICE),
            &crate::msgs::instantiate_msg::InstantiateMsg {
                croncat_factory_address: factory.to_string(),
            },
            &[],
            "create task handle tick 🤙",
            None,
        )
        .unwrap();

    let create_task_handle_tick_res = app.execute_contract(
        Addr::unchecked(ALICE),
        example_address.clone(),
        &crate::msgs::execute_msg::ExecuteMsg::MakeCroncatTickTask {},
        coins(1_000_000, DENOM).as_slice(),
    );

    assert!(create_task_handle_tick_res.is_ok());

    // Register an agent who will execute the task
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
    // auctions are in our example
    let mut mock_auctions: Vec<Auction> = app
        .wrap()
        .query_wasm_smart(example_address.clone(), &Auctions {})
        .expect("Did not retrieve mock auctions properly");

    // Before the agent calls CronCat, fulfilling the task, we have three auctions
    assert_eq!(mock_auctions.len(), 3usize);

    app.update_block(|block| add_seconds_to_block(block, 10));
    app.update_block(|block| increment_block_height(block, Some(2)));

    // Run proxy_call from the agent
    let proxy_call_res = app.execute_contract(
        Addr::unchecked(AGENT),
        manager,
        &ProxyCall { task_hash: None },
        &[],
    );

    assert!(proxy_call_res.is_ok());

    // Check that one auction has been removed due to logic
    // inside the tick method
    mock_auctions = app
        .wrap()
        .query_wasm_smart(example_address, &Auctions {})
        .expect("Did not retrieve mock auctions properly");

    // We fast-forwarded 10 seconds, meaning one auction expired, leaving two
    assert_eq!(mock_auctions.len(), 2usize);
}
