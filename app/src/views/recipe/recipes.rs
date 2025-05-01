use api::get_recipes;
use dioxus::{logger::tracing::info, prelude::*};

use crate::components::RecipePreview;

#[component]
pub fn Recipes() -> Element {
    let recipes = use_server_future(async move || get_recipes().await.unwrap())?;
    let recipes_read = recipes.read();

    rsx! {
        main {
            class: "main column gapMedium",
            section {
                class: "section column",
                h1 { class: "textLarge", "Recipes" }

                div {
                    class: "row gapMedium centerRow flexWrap",
                    for recipe in recipes_read.as_ref().unwrap().iter() {
                        // info!("Recipe: {:#?} id {}", recipe, recipe.id.clone());
    
                        RecipePreview { id: recipe.id, name: recipe.name.clone(), summary: recipe.summary.clone(), source: recipe.source.clone() }
                    }
                }
            }
        }
    }
}
