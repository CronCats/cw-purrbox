use cosmwasm_schema::write_api;

use boolean_contract::msgs::execute_msg::ExecuteMsg;
use boolean_contract::msgs::instantiate_msg::InstantiateMsg;
use boolean_contract::msgs::query_msg::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
