use api::auth::ServerLoginToken;
use dioxus::{
    hooks::{use_effect, use_resource},
    prelude::{use_server_future, RenderError}, signals::Writable,
};
use dioxus_sdk::storage::{LocalStorage, use_synced_storage};

use crate::{LOGIN_TOKEN_GLOBAL, constants::LOGIN_TOKEN_KEY};

pub fn is_logged_in() -> Result<bool, RenderError> {
    let local_token = LOGIN_TOKEN_GLOBAL();

    let Some(local_token) = local_token else {
        return Ok(false);
    };

    let is_token_valid = use_resource(move || {
        let local_token = local_token.clone();

        async move {
            let val = api::auth::server_is_logged_in(local_token).await;
            val.unwrap()
        }
    })
    .suspend()?;

    let is_valid = is_token_valid();
    if !is_valid {
        logout()
    }

    Ok(is_valid)
}

pub fn logout() {
    let mut cached_login_token = use_synced_storage::<LocalStorage, Option<ServerLoginToken>>(
        LOGIN_TOKEN_KEY.to_string(),
        || None,
    );

    *cached_login_token.write() = None;
    *LOGIN_TOKEN_GLOBAL.write() = None;
    
}

pub fn round_to_decimals(num: f32, decimals: i32) -> f32 
{
    let factor = 10.0_f32.powi(decimals);
    let result = (num * factor).round() / factor;
    result
}
