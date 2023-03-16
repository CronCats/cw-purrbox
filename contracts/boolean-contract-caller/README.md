# Boolean contract caller

This contract calls the `boolean-contract`, but it does so by creating a CronCat task, and instructing it to call the `toggle` method. (This flips the only state variable from true to false, and vice versa.) 

When you instantiate the contract, you provide the CronCat Factory contract address as well as the boolean contract address.

To create the task, call the method `make_croncat_toggle_task` with no parameters.

This example is meant to demonstrate an extremely simple CronCat task that sends a Wasm Execute message. CronCat tasks can call any smart contract using the approach here. 
