use cosmwasm_std::{coin, coins, Addr, Empty};
use cw_croncat_core::msg::TaskResponse;
use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

use crate::msg::ExecuteMsg;

const ADMIN: &str = "admin";
const AGENT: &str = "cosmos1a7uhnpqthunr2rzj0ww0hwurpn42wyun6c5puz";
const NATIVE_DENOM: &str = "ujunox";

fn mock_app() -> App {
    AppBuilder::new().build(|router, _, storage| {
        let accounts: Vec<(u128, String)> = vec![
            (6_000_000, ADMIN.to_string()),
            (1_000_000, AGENT.to_string()),
        ];
        for (amt, address) in accounts.iter() {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(address),
                    vec![coin(amt.clone(), NATIVE_DENOM.to_string())],
                )
                .unwrap();
        }
    })
}

fn croncat_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw_croncat::entry::execute,
        cw_croncat::entry::instantiate,
        cw_croncat::entry::query,
    )
    .with_reply(cw_croncat::entry::reply);
    Box::new(contract)
}

fn dao_versioner_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

fn rules_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw_rules::contract::execute,
        cw_rules::contract::instantiate,
        cw_rules::contract::query,
    );
    Box::new(contract)
}

fn registry_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw_code_id_registry::contract::execute,
        cw_code_id_registry::contract::instantiate,
        cw_code_id_registry::contract::query,
    );
    Box::new(contract)
}

#[test]
fn proxy_call_not_blocked() {
    let mut app = mock_app();
    let versioner_id = app.store_code(dao_versioner_contract());
    let registrar_id = app.store_code(registry_contract());
    let croncat_id = app.store_code(croncat_contract());
    let rules_id = app.store_code(rules_contract());
    let sender = Addr::unchecked(ADMIN);

    let rules_addr = app
        .instantiate_contract(
            rules_id,
            sender.clone(),
            &cw_rules_core::msg::InstantiateMsg {},
            &[],
            "cw-rules",
            None,
        )
        .unwrap();

    let croncat_addr = app
        .instantiate_contract(
            croncat_id,
            sender.clone(),
            &cw_croncat_core::msg::InstantiateMsg {
                denom: "ujunox".to_owned(),
                cw_rules_addr: rules_addr.to_string(),
                owner_id: None,
                gas_base_fee: None,
                gas_action_fee: None,
                gas_fraction: None,
                agent_nomination_duration: None,
            },
            &[],
            "croncat",
            None,
        )
        .unwrap();
    let registrar_addr = app
        .instantiate_contract(
            registrar_id,
            sender.clone(),
            &cw_code_id_registry::msg::InstantiateMsg {
                admin: sender.to_string(),
            },
            &[],
            "registrar",
            None,
        )
        .unwrap();
    let versioner_addr = app
        .instantiate_contract(
            versioner_id,
            sender.clone(),
            &crate::msg::InstantiateMsg {
                registrar_addr: registrar_addr.to_string(),
                croncat_addr: croncat_addr.to_string(),
            },
            &[],
            "versioneer",
            None,
        )
        .unwrap();

    // register_code_id
    app.execute_contract(
        sender.clone(),
        registrar_addr.clone(),
        &cw_code_id_registry::msg::ExecuteMsg::Register {
            contract_name: "dao-contract".to_string(),
            version: "1".to_string(),
            chain_id: "uni-5".to_string(),
            code_id: croncat_id,
            checksum: "todo".to_string(),
        },
        &[],
    )
    .unwrap();

    // create versioner
    app.execute_contract(
        sender.clone(),
        versioner_addr.clone(),
        &ExecuteMsg::CreateVersioner {
            daodao_addr: "todo".to_string(),
            name: "dao-contract".to_string(),
            chain_id: "uni-5".to_string(),
        },
        &coins(1_000_000, NATIVE_DENOM),
    )
    .unwrap();

    // register agent
    app.execute_contract(
        Addr::unchecked(AGENT),
        croncat_addr.clone(),
        &cw_croncat_core::msg::ExecuteMsg::RegisterAgent {
            payable_account_id: None,
        },
        &[],
    )
    .unwrap();

    app.update_block(|b| {
        b.height += 1;
        b.time.plus_seconds(10);
    });

    app.execute_contract(
        Addr::unchecked(AGENT),
        croncat_addr.clone(),
        &cw_croncat_core::msg::ExecuteMsg::ProxyCall { task_hash: None },
        &[],
    )
    .unwrap();

    let tasks: Vec<TaskResponse> = app
        .wrap()
        .query_wasm_smart(
            croncat_addr,
            &cw_croncat_core::msg::QueryMsg::GetTasks {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert!(tasks.is_empty())
}
