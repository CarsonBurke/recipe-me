use std::time::{self, SystemTime, UNIX_EPOCH};

use dioxus::prelude::ServerFnError;

#[server]
pub async fn ping_self() -> Result<u128, ServerFnError> {
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    
    Ok(since_epoch.as_millis())
}

#[server]
pub async fn ping_net_server() -> Result<u128, ServerFnError> {

    let server_time = || {};
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let diff;

    Ok(diff)
}