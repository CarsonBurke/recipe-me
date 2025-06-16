use std::f32::EPSILON;

use api::{get_recipe, get_recipe_cuisines, get_recipe_diets, get_recipe_ingredients, get_recipe_meals};
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::{components::{collection::collections::CollectionPreviews, dialog::DialogWrapper, rating_static::RatingStatic, recipe::{comments::RecipeComments, filtered_public}}, data::partials::IngredientPartial, entities::recipe_collection, server::{self, recipe::create_recipe}, utils::round_to_decimals, views::{self, recipe::recipes}, Route};

#[component]
pub fn RecipeComp(id: ReadOnlySignal<i32>, is_local: bool) -> Element {
    println!("RecipePage: {id}");

    println!("Second check {}", id());

    /* web_sys::window().and_then(|win| Some(win.scroll_to_with_x_and_y(0.0, 0.0))); */

    // let id_d = use_memo(move || id);

    // let id = use_signal(|| id);
    let recipe = use_resource(move || {
        // let cloned_id = id();
        let cloned_id = id();
        println!("check 1 id: {cloned_id}");
        async move {
            println!("check 2 id: {cloned_id}");
            get_recipe(cloned_id).await.unwrap()
        }
    })
    .suspend()?;
    let recipe_read = recipe();
    println!("Read recipe id: {}", recipe_read.id);
    // let recipe_ref = recipe_read.as_ref().unwrap();

    let ingredients = use_resource(move || {
        // let cloned_id = id();
        let cloned_id = id();
        async move { get_recipe_ingredients(cloned_id).await.unwrap() }
    })
    .suspend()?;
    let ingredients_read = &*ingredients.read();
    // let ingredients_ref = ingredients_read.as_ref().unwrap();

    let cuisines = use_resource(move || {
        // let cloned_id = id();
        let cloned_id = id();
        async move { get_recipe_cuisines(cloned_id).await.unwrap() }
    })
    .suspend()?;
    let cuisines_read = &*cuisines.read();
    // let cuisines_ref = cuisines_read.as_ref().unwrap();

    let meals = use_resource(move || {
        // let cloned_id = id();
        let cloned_id = id();
        async move { get_recipe_meals(cloned_id).await.unwrap() }
    })
    .suspend()?;
    let meals_read = &*meals.read();
    // let meals_ref = meals_read.as_ref().unwrap();

    let diets = use_resource(move || {
        // let cloned_id = id();
        let cloned_id = id();
        async move { get_recipe_diets(cloned_id).await.unwrap() }
    })
    .suspend()?;
    let diets_read = &*diets.read();
    // let diets_ref = diets_read.as_ref().unwrap();

    struct LocalData {
        collection: Resource<recipe_collection::Model>,
    }

    fn get_local_data(is_local: bool) -> Option<LocalData> {
        if is_local {
            Some(LocalData {
                collection: use_resource(|| async move {
                    server::collection::get_collection(0).await.unwrap()
                }),
            })
        } else {
            None
        }
    }

    let local_data = get_local_data(is_local);

    let rating = recipe_read.total_rating as f32 / (recipe_read.ratings as f32 + EPSILON);

    let mut ingredients_mult = use_signal(|| 1.);

    /* use_effect(move || {
        let mult = ingredients_mult();
    }); */

    /* use_effect(move || {
        let id = id();

        web_sys::window().and_then(|win| Some(win.scroll_to_with_x_and_y(0.0, 0.0)));
    }); */

    let mut main = use_signal(|| None);

    rsx! {
        main {
            onmounted: move |cx| {
                println!("Mounted");
                main.set(Some(cx.data()));

                async move {
                    match main.cloned() {
                        Some(main) => {
                            println!("try to scroll to");
                            let _ = main.scroll_to(ScrollBehavior::Smooth).await;
                        }
                        None => ()
                    };
                }
            },
            class: "main",
            section {
                class: "section column gapMedium",
                div {
                    class: "column gapLarge borderBg2 round paddingLarge",
                    div {
                        class: "column centerRow gapMedium",
                        div {
                            class: "row spaceBetween gapMedium flexWrap",
                            div {
                                class: "column gapSmall",
                                h1 { class: "textXLarge", {recipe_read.name.clone()} }
                                if recipe_read.summary.len() != recipe_read.description.len() {
                                    p {
                                        {recipe_read.summary.clone()},
                                    }
                                }
                            }
                            div {
                                class: "row centerColumn gapMedium spaceBetween width100",
                                div {
                                    class: "row centerColumn gapMedium flexWrap",
                                    div {
                                        class: "row centerColumn gapSmall",
                                        if recipe_read.ratings == 0 {
                                            p { class: "textSmall textWeak", "No ratings" }
                                        }
                                        else {
                                            RatingStatic {
                                                rating
                                            }
                                            p { class: "row textSmall textWeak", {format!("{}/5", round_to_decimals(rating, 1))} }
                                        }
                                    }
                                    p { class: "textSmall textWeak", {format!("{} ratings", recipe_read.ratings)} }
                                }
                                div {
                                    class: "row gapSmall",
                                    if let Some(local_data) = local_data {
                                        button {
                                            class: "button buttonBg2",
                                            onclick: move |_| async move {
                                                println!("Favourite recipe");
                                            },
                                            dioxus_free_icons::Icon { icon: ld_icons::LdHeart }
                                        }
                                    }
                                    else {
                                        button {
                                            class: "button buttonBg2",
                                            onclick: move |_| {
                                            let name = recipe_read.name.clone();
                                            let description = recipe_read.description.clone();
                                            let instructions = recipe_read.instructions.clone();
                                            
                                            async move {
                                                println!("Add to library");

                                                let server_ingredients = get_recipe_ingredients(id()).await.unwrap();
                                                let ingredients = server_ingredients.iter().map(|i| IngredientPartial::from(i)).collect::<Vec<IngredientPartial>>();

                                                let recipe_id = create_recipe(name, description, instructions, ingredients).await;

                                                println!("Created local recipe from recipe with id {}", recipe_id);
                                            }
                                            },
                                            dioxus_free_icons::Icon { icon: ld_icons::LdSave }
                                        }
                                    }
                                    DialogWrapper {
                                        header: rsx! {
                                            h1 { class: "textLarge", "Add to collection" }
                                        },
                                        button: rsx! {
                                            button {
                                                class: "button buttonBg2 textXSmall",
                                                onclick: move |_| {
                                                    println!("Add to collection")
                                                },
                                                dioxus_free_icons::Icon { icon: ld_icons::LdBookPlus }
                                            }
                                        },
                                        dialog: rsx! {
                                            div {
                                                class: "row overflowHorizontal gapSmall round",
                                                CollectionPreviews {  }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "row gapSmall centerColumn overflowHorizontal round",
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
                        class: "row centerRow width100",
                        div {
                            class: "sectionImage bg3 round",
                        }
                    }
                    p {
                        {recipe().description},
                    }
                    div {
                        class: "column gapMedium",
                        h2 { class: "textLarge",  "Ingredients"}
                        div {
                            class: "row gapLarge centerColumn widthFit",
                            div {
                                class: "row round bg2",
                                button {
                                    class: "buttonSmall buttonBg2",
                                    onclick: move |_| {
                                        if ingredients_mult() < 1. {
                                            ingredients_mult.set(ingredients_mult() + 0.25);
                                            return
                                        }

                                        ingredients_mult.set((ingredients_mult() as f32 + 1.).min(20.))
                                    },
                                    "+"
                                }
                                button {
                                    class: "buttonSmall buttonBg2",
                                    onclick: move |_| {
                                        if ingredients_mult() <= 1. {
                                            ingredients_mult.set((ingredients_mult() - 0.25).max(0.25));
                                            return
                                        }

                                        ingredients_mult.set((ingredients_mult() - 1.).max(1.))
                                    },
                                    "-"
                                }
                            }
                            h3 {
                                class: "textSmall",
                                {format!("{} Servings", ingredients_mult())}
                            }
                        }
                        div {
                            class: "column gapSmall",
                            for ingredient in ingredients_read {
                                p {
                                    class: "textSmall",
                                    {format!("{} {} {}", round_to_decimals(ingredient.amount.clone() * ingredients_mult() as f32, 2), ingredient.description.clone(), ingredient.name.clone())}
                                }
                            }
                        }
                    }
                    div {
                        class: "column gapSmall",
                        h2 { class: "textLarge",  "Instructions"}
                        p {
                            {recipe().instructions},
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
                    div {
                        class: "row overflowHorizontal gapMedium",
                        filtered_public::FilteredRecipes {
                            diet_id: Some(diet.id),
                            limit: Some(8),
                            recipe_select: false,
                        }
                    }
                }
                for cuisine in cuisines_read {
                    h2 {
                        class: "textLarge",
                        {format!("{} Recipes", cuisine.name.clone())},
                    }
                    div {
                        class: "row overflowHorizontal gapMedium",
                        filtered_public::FilteredRecipes {
                            cuisine_id: Some(cuisine.id),
                            limit: Some(8),
                            recipe_select: false,
                        }
                    }
                }
                for meal in meals_read {
                    h2 {
                        class: "textLarge",
                        {format!("{} Recipes", meal.name.clone())},
                    }
                    div {
                        class: "row overflowHorizontal gapMedium",
                        filtered_public::FilteredRecipes {
                            meal_id: Some(meal.id),
                            limit: Some(8),
                            recipe_select: false,
                        }
                    }
                }
                div {
                    class: "row centerRow",
                    button {
                        class: "buttonBg2 button",
                        onclick: move |_| {
                            navigator().go_back();
                        },
                        dioxus_free_icons::Icon { icon: ld_icons::LdArrowLeft }
                        "Back"
                    }
                }
            }
        }
    }
}
