use crate::entry_points::execute::make_croncat_toggle_task::BooleanContractExecuteMsg;
use crate::msgs::execute_msg::ExecuteMsg::MakeCroncatToggleTask;
use crate::tests::contracts;
use cosmwasm_std::CosmosMsg::Wasm;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{coins, from_binary, to_binary, Addr, BankMsg};
use croncat_integration_testing::test_helpers::{set_up_croncat_contracts, CronCatTestEnv};
use croncat_integration_testing::{CronCatTaskExecutionInfo, ALICE, BOB, DENOM, VERSION};
use croncat_integration_utils::{
    CronCatAction, CronCatBoundary, CronCatBoundaryHeight, CronCatInterval, CronCatTaskRequest,
};
use croncat_sdk_tasks::msg::TasksExecuteMsg::CreateTask;
use croncat_sdk_tasks::types::Task;
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
    let action = CronCatAction {
        msg: BankMsg::Send {
            to_address: Addr::unchecked(BOB).to_string(),
            amount: coins(10, DENOM),
        }
        .into(),
        gas_limit: Some(100_000),
    };

    let task = CronCatTaskRequest {
        interval: CronCatInterval::Once,
        boundary: Some(CronCatBoundary::Height(CronCatBoundaryHeight {
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
            &CreateTask {
                task: Box::new(task.clone()),
            },
            &coins(300_000, DENOM),
        )
        .expect("Couldn't create first task");

    let task_info: CronCatTaskExecutionInfo = from_binary(&create_task_res.data.unwrap()).unwrap();

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
    } = set_up_croncat_contracts(None);

    // Deploy boolean contract
    let mut code_id = app.store_code(contracts::boolean_contract());

    let boolean_instantiate_res = app.instantiate_contract(
        code_id,
        Addr::unchecked(ALICE),
        &cw_boolean_contract::msgs::instantiate_msg::InstantiateMsg {},
        &[],
        "boolean contract 🤙",
        None,
    );

    let boolean_address =
        boolean_instantiate_res.expect("Error instantiating the boolean contract");

    // Deploy this example contract
    code_id = app.store_code(contracts::boolean_contract_caller());
    let example_address = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(ALICE),
            &crate::msgs::instantiate_msg::InstantiateMsg {
                croncat_factory_address: factory.to_string(),
                boolean_address: boolean_address.to_string(),
            },
            &[],
            "boolean contract caller 🤙",
            None,
        )
        .unwrap();

    let toggle_task_res = app.execute_contract(
        Addr::unchecked(ALICE),
        example_address.clone(),
        &MakeCroncatToggleTask {},
        coins(1_000_000, DENOM).as_slice(),
    );

    assert!(toggle_task_res.is_ok());

    let toggle_task_binary_data = toggle_task_res
        .unwrap()
        .data
        .expect("Could not get the data response");

    let created_task_info: CronCatTaskExecutionInfo =
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
                    interval: CronCatInterval::Block(1),
                    boundary: CronCatBoundary::Height(CronCatBoundaryHeight {
                        start: None,
                        end: None,
                    }),
                    stop_on_fail: true,
                    actions: vec![CronCatAction {
                        msg: Wasm(Execute {
                            contract_addr: boolean_address.clone().to_string(),
                            msg: to_binary(&BooleanContractExecuteMsg::Toggle {})
                                .expect("Could not turn the toggle task into binary"),
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
    // Ideally we'd like to check the transaction index as well, but it doesn't seem to be available in cw-multi-test yet.
}
