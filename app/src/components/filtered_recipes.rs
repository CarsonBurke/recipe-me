use std::f32::EPSILON;

use api::{FilteredRecipesParams, get_filtered_recipes};
use dioxus::prelude::*;

use crate::components::recipe::preview::RecipePreview;

#[derive(Clone, Debug, Copy, PartialEq, Default)]
pub struct Params {}

#[component]
pub fn FilteredRecipes(
    cuisine_id: Option<i32>,
    diet_id: Option<i32>,
    meal_id: Option<i32>,
    ingredient_id: Option<i32>,
    limit: Option<u64>,
    author_id: Option<i32>,
    public: Option<bool>,
    collection_id: Option<i32>,
) -> Element {
    let recipes = use_resource(move || {
        let params = FilteredRecipesParams {
            cuisine_id: cuisine_id.clone(),
            diet_id: diet_id.clone(),
            ingredient_id: ingredient_id.clone(),
            meal_id: meal_id.clone(),
            limit: limit.clone().unwrap_or(50),
            author_id: author_id.clone(),
            public: public.clone(),
            collection_id: collection_id.clone(),
            page_offset: Some(0),
        };
        async move { get_filtered_recipes(params).await.unwrap() }
    }).suspend()?;

    if recipes().is_empty() {
        return rsx! {
            p { class: "textMedium", "No recipes found" }
        };
    }

    rsx! {
        for recipe in recipes().iter() {

            RecipePreview { id: recipe.id, name: recipe.name.clone(), summary: recipe.summary.clone(), source: recipe.source.clone(), rating: (recipe.total_rating as f32) / (recipe.ratings as f32 + EPSILON ) }
        }
    }
}
