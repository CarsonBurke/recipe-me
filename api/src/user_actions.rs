use dioxus::prelude::*;

#[server]
pub async fn create_recipe(name: String, description: String, instructions: String) -> Result<String, ServerFnError> {
    Ok("hello".to_string())
}