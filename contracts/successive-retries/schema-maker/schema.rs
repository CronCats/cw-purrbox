use cosmwasm_schema::write_api;

use successive_retries::msgs::execute_msg::ExecuteMsg;
use successive_retries::msgs::instantiate_msg::InstantiateMsg;
use successive_retries::msgs::query_msg::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
