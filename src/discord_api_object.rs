use serde::{Deserialize, Serialize};

// ===========GatewayObject===========
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct GatewayObject {
    pub url: String,
    pub shards: i32,
    pub session_start_limit: SessionStartLimit,
}
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct SessionStartLimit {
    pub total: i32,
    pub remaining: i32,
    pub reset_after: i32,
    pub max_concurrency: i32,
}
// ===================================
