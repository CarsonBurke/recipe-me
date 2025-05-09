use api::{FilteredRecipesParams, get_filtered_recipes};
use dioxus::prelude::*;

use crate::components::RecipePreview;

#[derive(Clone, Debug, Copy, PartialEq, Default)]
pub struct Params {
    pub cuisine_id: Option<i32>,
    pub diet_id: Option<i32>,
    pub meal_id: Option<i32>,
    pub ingredient_id: Option<i32>,
    pub limit: Option<u64>,
}

#[component]
pub fn FilteredRecipes(params: Params) -> Element {
    let recipes = use_server_future(move || {
        let params = FilteredRecipesParams {
            cuisine_id: params.cuisine_id.clone(),
            diet_id: params.diet_id.clone(),
            ingredient_id: params.ingredient_id.clone(),
            meal_id: params.meal_id.clone(),
            limit: params.limit.clone().unwrap_or(50),
            ..Default::default()
        };
        async move { get_filtered_recipes(params).await.unwrap() }
    })?;
    let recipes_read = recipes.read();

    rsx! {
        div {
            class: "row gapMedium centerRow flexWrap",
            for recipe in recipes_read.as_ref().unwrap().iter() {

                RecipePreview { id: recipe.id, name: recipe.name.clone(), summary: recipe.summary.clone(), source: recipe.source.clone(), rating: (recipe.total_rating / recipe.ratings) as f32 }
            }
        }
    }
}
