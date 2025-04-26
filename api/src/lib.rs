//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

mod data;
mod sample;

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[server(Recipes)]
pub async fn recipes() -> Result<String, ServerFnError> {
    Ok("recipes".to_string())
}