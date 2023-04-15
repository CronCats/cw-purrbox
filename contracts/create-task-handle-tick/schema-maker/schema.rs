use cosmwasm_schema::write_api;

use create_task_handle_tick::msgs::execute_msg::ExecuteMsg;
use create_task_handle_tick::msgs::instantiate_msg::InstantiateMsg;
use create_task_handle_tick::msgs::query_msg::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
