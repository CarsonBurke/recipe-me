use std::{collections::HashSet, f32::EPSILON};

use api::{get_filtered_recipes, FilteredRecipesParams};
use dioxus::{logger::tracing::info, prelude::*};

use crate::{
    components::recipe::preview::{RecipePreview, Selected},
    data::partials::IngredientPartial,
    server,
};

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
    let recipes = use_resource(move || async move {
        let params = FilteredRecipesParams {
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

        get_filtered_recipes(params).await.unwrap()
    })
    .suspend()?;

    if recipes().is_empty() {
        return rsx! {
            p { class: "textMedium", "No recipes found, check your internet connection" }
        };
    }

    let z = recipes();

    let selected_set: Signal<HashSet<i32>> = use_signal(|| HashSet::new());

    let create_from_selected_recipes = move |_| async move {
        println!("Add selected recipes {:?}", selected_set());

        for recipe_id in selected_set().iter() {
            println!("before new recipe");
            let new_recipe = server::recipe::recipe_from_public(*recipe_id).await;
            println!("new recipe: {:#?}", new_recipe);
        }

        navigator().go_back();
    };
    
    println!("recipe select: {}", recipe_select);
    rsx! {
        for recipe in z.iter() {

            RecipePreview {
                id: recipe.id,
                name: recipe.name.clone(),
                summary: recipe.summary.clone(),
                source: recipe.source.clone(),
                rating: (recipe.total_rating as f32) / (recipe.ratings as f32 + EPSILON),
                selected: match recipe_select { true => Selected::Unselected, false => Selected::NoSelect },
                selected_set,
                public: true,
            }
        }
        if !selected_set().is_empty() {
            div {
                class: "absBottom width100 row centerRow",
                button {
                    class: "button buttonBg2 widthFit",
                    onclick: create_from_selected_recipes,
                    "Add selected recipes"
                }
            }
        }
    }
}
