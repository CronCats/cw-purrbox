use cosmwasm_schema::write_api;

use cw_boolean_contract::msgs::execute_msg::ExecuteMsg;
use cw_boolean_contract::msgs::instantiate_msg::InstantiateMsg;
use cw_boolean_contract::msgs::query_msg::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
