use api::auth::ServerLoginToken;
use dioxus::{
    hooks::{use_effect, use_resource},
    prelude::{use_server_future, RenderError}, signals::Writable,
};
use dioxus_sdk::storage::{LocalStorage, use_synced_storage};

pub fn round_to_decimals(num: f32, decimals: i32) -> f32 
{
    let factor = 10.0_f32.powi(decimals);
    let result = (num * factor).round() / factor;
    result
}
