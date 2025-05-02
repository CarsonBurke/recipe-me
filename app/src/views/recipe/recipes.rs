use api::{FilteredRecipesParams, get_filtered_recipes, get_recipes};
use dioxus::{logger::tracing::info, prelude::*};

use crate::{components::RecipePreview, Route};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct RecipeFilterParams {
    pub cousine_id: Option<i32>,
    pub limit: Option<u64>,
    pub page_offset: u64,
}

impl RecipeFilterParams {
    pub fn into_filtered_recipes_params(self) -> FilteredRecipesParams {
        FilteredRecipesParams {
            cousine_id: self.cousine_id,
            limit: self.limit.unwrap_or(50),
            ..Default::default()
        }
    }
}

#[component]
pub fn Recipes(filter_params: RecipeFilterParams) -> Element {
    let route = use_route::<Route>();
    info!("Route {:#?}", route);

    info!("Client filter params {:#?}", filter_params);
    let recipes = use_server_future(move || {
        let params =
            <RecipeFilterParams as Clone>::clone(&filter_params).into_filtered_recipes_params();
        async move {
            get_filtered_recipes(params)
            .await
            .unwrap()
        }
    })?;
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
