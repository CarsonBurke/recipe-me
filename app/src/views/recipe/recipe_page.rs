use api::{
    get_recipe, get_recipe_cuisines, get_recipe_diets, get_recipe_ingredients, get_recipe_meals,
    get_recipes,
};
use dioxus::{logger::tracing::info, prelude::*};

use crate::{
    components::{
        filtered_recipes::{self, FilteredRecipes}, recipe::comments::RecipeComments, RatingStatic, RecipePreview
    }, views::recipe::recipes::{self, RecipeFilterParams}, Route
};

#[component]
pub fn RecipePage(id: ReadOnlySignal<i32>) -> Element {

    info!("RecipePage: {id}");

    // let id_d = use_memo(move || id);

    // let id = use_signal(|| id);
    let recipe = use_resource(move || {
        // let cloned_id = id();
        let cloned_id = id();
        println !("check 1 id: {cloned_id}");
        async move { println!("check 2 id: {cloned_id}"); get_recipe(cloned_id).await.unwrap() }
    }).suspend()?;
    let recipe_read = &*recipe.read();
    println!("Recipe: {:#?}", recipe_read);
    // let recipe_ref = recipe_read.as_ref().unwrap();

    let ingredients = use_resource(move || {
        // let cloned_id = id();
        let cloned_id = id();
        async move { get_recipe_ingredients(cloned_id).await.unwrap() }
    }).suspend()?;
    let ingredients_read = &*ingredients.read();
    // let ingredients_ref = ingredients_read.as_ref().unwrap();

    let cuisines = use_resource(move || {
        // let cloned_id = id();
        let cloned_id = id();
        async move { get_recipe_cuisines(cloned_id).await.unwrap() }
    }).suspend()?;
    let cuisines_read = &*cuisines.read();
    // let cuisines_ref = cuisines_read.as_ref().unwrap();

    let meals = use_resource(move || {
        // let cloned_id = id();
        let cloned_id = id();
        async move { get_recipe_meals(cloned_id).await.unwrap() }
    }).suspend()?;
    let meals_read = &*meals.read();
    // let meals_ref = meals_read.as_ref().unwrap();

    let diets = use_resource(move || {
        // let cloned_id = id();
        let cloned_id = id();
        async move { get_recipe_diets(cloned_id).await.unwrap() }
    }).suspend()?;
    let diets_read = &*diets.read();
    // let diets_ref = diets_read.as_ref().unwrap();

    let rating = recipe_read.total_rating as f32 / recipe_read.ratings as f32;

    let mut ingredients_mult = use_signal(|| 1);

    use_effect(move || {
        let mult = ingredients_mult();
    });

    /* use_effect(move || {
        let id = id();
    }); */

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapMedium",
                div {
                    class: "column gapLarge borderBg2 round paddingLarge",
                    div {
                        class: "column centerRow gapSmall",
                        div {
                            class: "row gapLarge",
                            h1 { class: "textXLarge", {recipe_read.name.clone()} }
                            div {
                                class: "row centerColumn gapMedium",
                                RatingStatic {
                                    rating
                                }
                                p { class: "textSmall textWeak", {format!("{rating:.1} / 5")} }
                            }
                        }

                        p {
                            {recipe_read.summary.clone()},
                        }
                    }
                    div {
                        class: "row gapLarge flexWrap centerColumn",
                        div {
                            class: "column gapSmall",
                            p { class: "textSmall textWeak", "Meal" },
                            div {
                                class: "row gapSmall",
                                for meal in meals_read {
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
                            p { class: "textSmall textWeak", "Diet" },
                            div {
                                class: "row gapSmall",
                                for diet in diets_read {
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
                            p { class: "textSmall textWeak", "Cuisine" },
                            div {
                                class: "row gapSmall",
                                for cuisine in cuisines_read {
                                    Link {
                                        to: Route::Recipes {
                                            query: recipes::Query {
                                                cuisine_id: Some(cuisine.id),
                                                ..Default::default()
                                            }
                                        },
                                        class: "pill textSmall button buttonBg2",
                                        {cuisine.name.clone()}
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "sectionImage bg3 round",
                    }
                    p {
                        {recipe_read.description.clone()},
                    }
                    div {
                        class: "column gapMedium",
                        h2 { class: "textLarge",  "Ingredients"}
                        div {
                            class: "row gapLarge centerColumn widthFit",
                            h3 {
                                class: "textSmall",
                                {format!("{} Servings", ingredients_mult())}
                            }
                            div {
                                class: "row gapSmall round bg2",
                                button {
                                    class: "button buttonBg2",
                                    onclick: move |_| ingredients_mult.set((ingredients_mult() + 1).min(20)),
                                    "+"
                                }
                                button {
                                    class: "button buttonBg2",
                                    onclick: move |_| ingredients_mult.set((ingredients_mult() - 1).max(1)),
                                    "-"
                                }
                            }
                        }
                        div {
                            class: "column gapSmall",
                            for ingredient in ingredients_read {
                                p {
                                    class: "textSmall",
                                    {format!("{} {} {}", (ingredient.amount.clone() * ingredients_mult()).to_string(), ingredient.description.clone(), ingredient.name.clone())}
                                }
                            }
                        }
                    }
                    div {
                        class: "column gapSmall",
                        h2 { class: "textLarge",  "Instructions"}
                        p {
                            {recipe_read.instructions.clone()},
                        }
                    }
                }
            }
            section {
                class: "section column gapMedium",
                div {
                    class: "column gapMedium borderBg2 round paddingLarge",
                    h1 {
                        class: "textXLarge",
                        "Comments"
                    }
                    RecipeComments {
                        recipe_id: id(),
                    }
                }
            }
            section {
                class: "section column gapMedium",
                h1 {
                    class: "textXLarge",
                    "More Recipes"
                }
                for diet in diets_read {
                    h2 {
                        class: "textLarge",
                        {format!("{} Recipes", diet.name.clone())},
                    }
                    FilteredRecipes {
                        params: filtered_recipes::Params {
                            diet_id: Some(diet.id),
                            limit: Some(4),
                            ..Default::default()
                        }
                    }
                }
                for cuisine in cuisines_read {
                    h2 {
                        class: "textLarge",
                        {format!("{} Recipes", cuisine.name.clone())},
                    }
                    FilteredRecipes {
                        params: filtered_recipes::Params {
                            cuisine_id: Some(cuisine.id),
                            limit: Some(4),
                            ..Default::default()
                        }
                    }
                }
                for meal in meals_read {
                    h2 {
                        class: "textLarge",
                        {format!("{} Recipes", meal.name.clone())},
                    }
                    FilteredRecipes {
                        params: filtered_recipes::Params {
                            meal_id: Some(meal.id),
                            limit: Some(4),
                            ..Default::default()
                        }
                    }
                }
            }
        }
    }
}
