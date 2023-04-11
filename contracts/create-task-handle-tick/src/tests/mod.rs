mod create_task_handle_tick;
mod contracts;
mod test_helpers;

/// We set this to "TOKEN" to match the denom here:
/// https://github.com/CosmWasm/cosmwasm/blob/32f308a1a56ae5b8278947891306f7a374c3df94/packages/vm/src/environment.rs#L383
pub const DENOM: &str = "TOKEN";

// Test accounts
pub const ALICE: &str = "cosmos1a7uhnpqthunr2rzj0ww0hwurpn42wyun6c5puz";
pub const BOB: &str = "cosmos17muvdgkep4ndptnyg38eufxsssq8jr3wnkysy8";
pub const CHARLIZE: &str = "cosmos1qxywje86amll9ptzxmla5ah52uvsd9f7drs2dl";
pub const VERY_RICH: &str = "cosmos1c3cy3wzzz3698ypklvh7shksvmefj69xhm89z2";
pub const AGENT: &str = "cosmos1a7uhnpqthunr2rzj0ww0hwurpn42wyun6c5puz";

// Other constants
pub const VERSION: &str = "0.1";
pub const PAUSE_ADMIN: &str = "juno18rzed6k8qupl209f3myhp6hlt6d4gldskyjjrdnc2q9qyrntwutqc2cntn";
