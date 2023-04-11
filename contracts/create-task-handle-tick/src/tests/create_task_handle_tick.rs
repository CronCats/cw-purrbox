use crate::msgs::execute_msg::ExecuteMsg::Tick;
use crate::tests::test_helpers::{set_up_croncat_contracts, CronCatTestEnv, add_seconds_to_block, increment_block_height};
use crate::tests::{contracts, ALICE, BOB, DENOM, VERSION, AGENT};
use cosmwasm_std::CosmosMsg::Wasm;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{coins, from_binary, to_binary, Addr, BankMsg};
use croncat_manager::msg::ExecuteMsg::ProxyCall;
use croncat_sdk_agents::msg::ExecuteMsg::RegisterAgent;
use croncat_sdk_tasks::types::{
    Action, Boundary, BoundaryHeight, Interval, Task, TaskExecutionInfo, TaskRequest,
};
use cw_multi_test::Executor;
use crate::msgs::query_msg::QueryMsg::Auctions;
use crate::state::Auction;

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
    } = set_up_croncat_contracts();

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

    // remember, to see this you gotta:
    // cargo test -- --nocapture
    println!(
        "Directly call CronCat Tasks contract\ntask_info: {:?}",
        task_info
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
    } = set_up_croncat_contracts();

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

    let toggle_task_binary_data = create_task_handle_tick_res
        .unwrap()
        .data
        .expect("Could not get the data response");

    let created_task_info: TaskExecutionInfo =
        serde_json::from_slice(toggle_task_binary_data.0.as_slice()).unwrap();

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
    // Ideally we'd like to check the transaction index as well, but it seems that cw-multi-test hardcodes this to 0, so we'll ignore in tests.
}

#[test]
fn tick_directly() {
    let CronCatTestEnv {
        mut app,
        factory,
        manager: _,
        tasks: _,
        agents: _,
    } = set_up_croncat_contracts();

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
    assert!(create_tick_res.is_ok());
}

#[test]
fn create_task_proxy_call() {
    let CronCatTestEnv {
        mut app,
        factory,
        manager,
        tasks: _,
        agents,
    } = set_up_croncat_contracts();

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
    let agent_registration_res = app.execute_contract(Addr::unchecked(AGENT), agents, &RegisterAgent {
        payable_account_id: None,
    }, &[]);

    assert!(agent_registration_res.is_ok());

    // Before executing the CronCat task, let's check how many
    // auctions are in our example
    let mut mock_auctions: Vec<Auction> = app.wrap().query_wasm_smart(example_address.clone(), &Auctions {}).expect("Did not retrieve mock auctions properly");

    // Before the agent calls CronCat, fulfilling the task, we have three auctions
    assert_eq!(mock_auctions.len(), 3usize);

    app.update_block(|block| add_seconds_to_block(block, 10));
    app.update_block(|block| increment_block_height(block, Some(2)));

    // Run proxy_call from the agent
    let proxy_call_res = app.execute_contract(Addr::unchecked(AGENT), manager, &ProxyCall { task_hash: None }, &[]);

    assert!(proxy_call_res.is_ok());

    // Check that one auction has been removed due to logic
    // inside the tick method
    mock_auctions = app.wrap().query_wasm_smart(example_address, &Auctions {}).expect("Did not retrieve mock auctions properly");

    // We fast-forwarded 10 seconds, meaning one auction expired, leaving two
    assert_eq!(mock_auctions.len(), 2usize);
}