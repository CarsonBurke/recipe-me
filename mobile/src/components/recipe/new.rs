use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::{data::partials::IngredientPartial, server, views, Route};

#[component]
pub fn NewRecipe(default_name: Option<String>, default_description: Option<String>, default_ingredients: Option<Vec<IngredientPartial>>, default_instructions: Option<String>) -> Element {
    let mut name = use_signal(|| default_name.unwrap_or_default());
    let mut description = use_signal(|| default_description.unwrap_or_default());
    let mut ingredients = use_signal::<Vec<IngredientPartial>>(|| default_ingredients.unwrap_or_default());
    let mut instructions = use_signal(|| default_instructions.unwrap_or_default());

    fn soft_can_create(name: String, description: String, instructions: String) -> bool {
        !name.is_empty() && !description.is_empty() && !instructions.is_empty()
    }

    rsx! {
        form {
            onsubmit: move |_| async move {
                if !soft_can_create(name(), description(), instructions()) {
                    return
                }
                
                println!("Creating recipe");

                let recipe_id = server::recipe::create_recipe(name(), description(), instructions(), ingredients()).await;

                let Ok(recipe_id) = recipe_id else {
                    println!("Failed to create recipe, err {:?}", recipe_id);
                    return
                };

                println!("created recipe with id {}", recipe_id);

                navigator().push(Route::Recipes { query: views::recipe::recipes::Query::default() });   

                /* let recipe_id = user_actions::create_recipe(login_token, name(), description(), instructions(), ingredients()).await;

                let Ok(recipe_id) = recipe_id else {
                    println!("Failed to create recipe, err {:?}", recipe_id);
                    return
                };

                println!("created recipe with id {}", recipe_id);

                navigator().push(Route::AccountRecipes { query: account::recipes::Query::default() }); */
            },
            class: "column gapLarge paddingLarge round bg2 centerColumn",
            h1 {class: "textLarge", "New recipe" },
            div {
                class: "column gapMedium centerColumn",
                div {
                    class: "column gapSmall",
                    label { class: "textSmall", "Enter a name" },
                    input {
                        placeholder: "Recipe name",
                        type: "text",
                        class: "input bg3 borderBg4",
                        oninput: move |e| name.set(e.value().clone()),
                    }
                }
                div {
                    class: "column gapSmall",
                    label { class: "textSmall", "Enter a description" },
                    textarea { class: "input bg3 borderBg4", oninput: move |e| description.set(e.value().clone()), placeholder: "Recipe description" }
                }
                div {
                    class: "column gapMedium",
                    p { class: "textSmall", "Enter ingredients" }
                    for (i, mut new_ingredient) in ingredients().into_iter().enumerate() {
                        div {
                            class: "row gapSmall centerColumn",
                            input {
                                placeholder: "2",
                                type: "number",
                                min: 0,
                                max: 1000,
                                class: "bg3 borderBg4 inputShort",
                                size: "3",
                                oninput: move |e| new_ingredient.amount = e.value().parse().unwrap_or(1.),
                            }
                            input {
                                placeholder: "tbsp, minced",
                                type: "text",
                                class: "bg3 borderBg4 inputShort",
                                size: "14",
                                oninput: move |e| new_ingredient.description = e.value(),
                            }
                            input {
                                placeholder: "garlic",
                                type: "text",
                                class: "inputShort bg3 borderBg4",
                                size: "14",
                                oninput: move |e| new_ingredient.name = e.value(),
                            }
                            button {
                                class: "buttonSmall buttonBg2",
                                type: "button",
                                onclick: move |_| {
                                    ingredients.remove(i);
                                },
                                dioxus_free_icons::Icon { icon: ld_icons::LdTrash },
                            }
                        }
                    }
                    button {
                        class: "button buttonBg3",
                        type: "button",
                        onclick: move |_| ingredients.push(IngredientPartial::default()),
                        dioxus_free_icons::Icon { icon: ld_icons::LdPlus },
                        "Add ingredient",
                    }
                }
                div {
                    class: "column gapSmall",
                    label { class: "textSmall", "Enter instructions" },
                    textarea { class: "input bg3 borderBg4", oninput: move |e| instructions.set(e.value().clone()), placeholder: "Recipe instructions" }
                }
            }
            button {
                class: "button buttonBg3",
                type: "submit",
                disabled: !soft_can_create(name(), description(), instructions()),
                dioxus_free_icons::Icon { icon: ld_icons::LdPlus }
                "Create recipe",
            }
        }
    }
}
