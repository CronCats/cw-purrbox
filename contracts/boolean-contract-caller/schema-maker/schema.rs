use cosmwasm_schema::write_api;

use boolean_contract_caller::msgs::execute_msg::ExecuteMsg;
use boolean_contract_caller::msgs::instantiate_msg::InstantiateMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
    }
}
