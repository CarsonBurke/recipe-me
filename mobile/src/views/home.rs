use api::FilteredRecipesParams;
use dioxus::prelude::*;

use crate::{components::filtered_recipes::FilteredRecipes, server};

#[component]
pub fn Home() -> Element {
    let x = use_resource(|| async move { server::ping_self().await });
    println!("x: {:?}", x);

    let recipes = use_resource(|| async move {
        api::get_filtered_recipes(FilteredRecipesParams {
            limit: 10,
            ..Default::default()
        })
        .await
    });
    println!("recipes: {:?}", recipes);

    rsx! {
        h1 { "Home" }

        div {
            class: "row overflowHorizontal gapMedium paddingLarge",
            FilteredRecipes {
                recipe_select: false,
            }
        }
    }
}
