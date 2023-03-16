use crate::entry_points::execute::execute;
use crate::entry_points::instantiate::instantiate;
use crate::entry_points::query::query;
use crate::msgs::execute_msg::ExecuteMsg;
use crate::msgs::query_msg::QueryMsg;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, to_binary};

#[test]
fn test_setting_getting() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("alice", &coins(6, "dps"));
    let inst_res = instantiate(deps.as_mut(), env.clone(), info.clone(), None);
    assert!(
        inst_res.is_ok(),
        "Instantiate didn't even work, brah. Ouch."
    );

    // Before we set anything, test that the boolean is_true is false
    let get_value_msg = QueryMsg::GetValue {};
    let mut query_res = query(deps.as_ref(), env.clone(), get_value_msg.clone());
    assert!(
        query_res.is_ok(),
        "Querying after instantiate should be fine."
    );
    assert_eq!(query_res.unwrap(), to_binary(&false).unwrap());

    // Set it to true using an execute message
    let set_value_msg = ExecuteMsg::SetValue { is_true: true };
    let exec_res = execute(deps.as_mut(), env.clone(), info, set_value_msg);
    assert!(
        exec_res.is_ok(),
        "Execute message changing to true should not fail"
    );

    // Check that it's true now
    query_res = query(deps.as_ref(), env, get_value_msg);
    assert!(
        query_res.is_ok(),
        "Querying after instantiate should be fine."
    );
    assert_eq!(query_res.unwrap(), to_binary(&true).unwrap());
}
