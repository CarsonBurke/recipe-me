use api::{
    get_recipe, get_recipe_cousines, get_recipe_diets, get_recipe_ingredients, get_recipe_meals,
    get_recipes,
};
use dioxus::{logger::tracing::info, prelude::*};

use crate::{
    components::{RatingStatic, RecipePreview}, views::recipe::recipes::{self, RecipeFilterParams}, Route
};

#[component]
pub fn RecipePage(id: i32) -> Element {
    let recipe = use_server_future(move || {
        let id = id;
        async move { get_recipe(id).await.unwrap() }
    })?;
    let recipe_read = recipe.read();
    let recipe_ref = recipe_read.as_ref().unwrap();

    let ingredients = use_server_future(move || {
        let id = id;
        async move { get_recipe_ingredients(id).await.unwrap() }
    })?;
    let ingredients_read = ingredients.read();
    let ingredients_ref = ingredients_read.as_ref().unwrap();

    let cousines = use_server_future(move || {
        let id = id;
        async move { get_recipe_cousines(id).await.unwrap() }
    })?;
    let cousines_read = cousines.read();
    let cousines_ref = cousines_read.as_ref().unwrap();

    let meals = use_server_future(move || {
        let id = id;
        async move { get_recipe_meals(id).await.unwrap() }
    })?;
    let meals_read = meals.read();
    let meals_ref = meals_read.as_ref().unwrap();

    let diets = use_server_future(move || {
        let id = id;
        async move { get_recipe_diets(id).await.unwrap() }
    })?;
    let diets_read = diets.read();
    let diets_ref = diets_read.as_ref().unwrap();

    let rating = recipe_ref.total_rating as f32 / recipe_ref.ratings as f32;

    rsx! {
        main {
            class: "main centerColumn",
            section {
                class: "section column gapMedium",
                div {
                    class: "column centerRow gapSmall",
                    div {
                        class: "row gapLarge",
                        h1 { class: "textLarge", {recipe_ref.name.clone()} }
                        div {
                            class: "row centerColumn gapMedium",
                            RatingStatic {  
                                rating
                            }
                            p { class: "textSmall textWeak", {format!("{rating:.1} / 5")} }
                        }
                    }
                    
                    p {
                        {recipe_ref.summary.clone()},
                    }
                }
                div {
                    class: "row gapLarge flexWrap centerColumn",
                    div {
                        class: "column gapSmall",
                        p { class: "textSmall textWeak", "Meal" },
                        div {
                            class: "row gapSmall",
                            for meal in meals_ref {
                                Link {
                                    to: Route::Recipes {
                                        query: recipes::Query {
                                            meal_id: Some(meal.id),
                                            ..Default::default()
                                        }
                                    },
                                    class: "pill textSmall button buttonBg2",
                                    {meal.name.clone()}
                                }
                            }
                        }
                    }
                    div {
                        class: "column gapSmall",
                        p { class: "textSmall textWeak", "Meal" },
                        div {
                            class: "row gapSmall",
                            for diet in diets_ref {
                                Link {
                                    to: Route::Recipes {
                                        query: recipes::Query {
                                            diet_id: Some(diet.id),
                                            ..Default::default()
                                        }
                                    },
                                    class: "pill textSmall button buttonBg2",
                                    {diet.name.clone()}
                                }
                            }
                        }
                    }
                    div {
                        class: "column gapSmall",
                        p { class: "textSmall textWeak", "Cousine" },
                        div {
                            class: "row gapSmall",
                            for cousine in cousines_ref {
                                {info!("Cousine: {:#?}", cousine);}
                                Link {
                                    to: Route::Recipes {
                                        query: recipes::Query {
                                            cousine_id: Some(cousine.id),
                                            ..Default::default()
                                        }
                                    },
                                    class: "pill textSmall button buttonBg2",
                                    {cousine.name.clone()}
                                }
                            }
                        }
                    }
                }
                div {
                    class: "sectionImage bg2 round",
                }
                p {
                    {recipe_ref.description.clone()},
                }
                div {
                    class: "column gapMedium",
                    h2 { class: "textMedium",  "Ingredients"}
                    div {
                        class: "column gapSmall",
                        for ingredient in ingredients_ref {
                            p {
                                class: "textSmall",
                                {format!("{} {} {}", ingredient.amount.clone().to_string(), ingredient.description.clone(), ingredient.name.clone())}
                            }
                        }
                    }
                }
                div {
                    class: "column gapSmall",
                    h2 { class: "textMedium",  "Instructions"}
                    p {
                        {recipe_ref.instructions.clone()},
                    }
                }
            }
        }
    }
}
