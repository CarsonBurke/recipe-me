use api::get_recipes;
use dioxus::{prelude::*, logger::tracing::{info}};

use crate::{components::recipe_preview::RecipePreview, recipe};

#[component]
pub fn RecipePage(id: i32) -> Element {
    let recipes = use_server_future(async move || get_recipes().await.unwrap())?;
    let recipes_read = recipes.read();

    rsx! {
        h1 { "Recipe Page with id {id}" }
        div {
            class: "row bg3 paddingMedium",
            "Value"
        }
        SuspenseBoundary {  
            fallback: |_| {
                rsx! {
                    div {
                        class: "row bg3 paddingMedium",
                        "Loading..."
                    }
                }
            },
            {
                for recipe in recipes_read.as_ref().unwrap().iter() {
                info!("Recipe: {:#?}", recipe);
                rsx! {
                    RecipePreview { name: recipe.name.clone() }
                };
            }}
        }
    }
}