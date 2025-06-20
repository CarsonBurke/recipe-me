use std::{collections::HashSet, f32::EPSILON};

use api::{get_filtered_recipes, FilteredRecipesParams};
use dioxus::prelude::*;

use crate::{components::recipe::preview::{RecipePreview, Selected}, server, views, Route};

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
    recipe_select: bool,
    collection_id: Option<i32>,
) -> Element {
    let recipes = use_resource(move || {
        
        async move {
            let params = server::recipe::FilteredRecipesParams {
                cuisine_id: cuisine_id.clone(),
                diet_id: diet_id.clone(),
                ingredient_id: ingredient_id.clone(),
                meal_id: meal_id.clone(),
                limit: limit.clone().unwrap_or(50),
                author_id: author_id.clone(),
                public: Some(recipe_select.clone()),
                collection_id: collection_id.clone(),
                page_offset: Some(0),
            };

            server::recipe::get_filtered_recipes(params).await   
        }
    })
    .suspend()?;

    if recipes().is_empty() {
        return rsx! {
            div {
                class: "column gapSmall centerColumn",
                p { class: "textMedium", "No recipes found" }
                Link {
                    class: "button buttonBg2",
                    to: Route::Recipes { query: views::recipe::recipes::Query::default() },
                    "Find a recipe"
                }
            }
        };
    }

    /* let selected_set: Signal<HashSet<i32>> = use_context_provider(|| use_signal(|| HashSet::new())); */
    let selected_set: Signal<HashSet<i32>> = use_signal(|| HashSet::new());
    println!("recipe select: {}", recipe_select);
    rsx! {
        for recipe in recipes().iter() {

            RecipePreview {
                id: recipe.id,
                name: recipe.name.clone(),
                summary: recipe.summary.clone(),
                source: recipe.source.clone(),
                total_rating: recipe.total_rating,
                ratings: recipe.ratings,
                selected: match recipe_select { true => Selected::Unselected, false => Selected::NoSelect },
                selected_set,
                public: false,
            }
        }
        if !selected_set().is_empty() {
            div {
                class: "width100 absBottom",
                button {
                    class: "button buttonBg2 width100",
                    onclick: move |_| {
                        println!("Add selected recipes {:?}", selected_set());
                    },
                    "Add selected recipes"
                }
            }
        }
    }
}
