use std::time::Duration;
pub const DB_PATH: &str = "./nodes.db";
pub const RETRIVE_NODES_URL: &str =
    "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";
pub const END_POINT_CHALLENGE: &str = "GET /nodes";
pub const IP: &str = "127.0.0.1";
pub const BIND: &str = "8080";
pub const TIME_UPDATE: Duration = Duration::from_secs(10);
