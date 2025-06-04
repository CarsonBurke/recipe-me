use api::user_actions::{self, NewIngredient};
use dioxus::{html::textarea, prelude::*};
use dioxus_free_icons::icons::ld_icons;

use crate::{Route};

#[component]
pub fn NewRecipe() -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut ingredients = use_signal::<Vec<NewIngredient>>(|| vec![]);
    let mut instructions = use_signal(|| "".to_string());

    fn soft_can_create(name: String, description: String, instructions: String) -> bool {
        !name.is_empty() && !description.is_empty() && !instructions.is_empty()
    }

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                form {
                    onsubmit: move |_| async move {
                        if !soft_can_create(name(), description(), instructions()) {
                            return
                        }
                        
                        println!("Creating recipe");

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
                            for mut new_ingredient in ingredients() {
                                div {
                                    class: "row gapSmall centerColumn",
                                    input {
                                        placeholder: "2",
                                        type: "number",
                                        min: 0,
                                        max: 1000,
                                        class: "bg3 borderBg4 inputShort",
                                        size: "3",
                                        oninput: move |e| new_ingredient.amount = e.value().parse().ok(),
                                    }
                                    input {
                                        placeholder: "tbsp, minced",
                                        type: "text",
                                        class: "bg3 borderBg4 inputShort",
                                        size: "14",
                                        oninput: move |e| new_ingredient.description = e.value(),
                                    }
                                    p { class: "textMedium", "of" }
                                    input {
                                        placeholder: "garlic",
                                        type: "text",
                                        class: "inputShort bg3 borderBg4",
                                        size: "14",
                                        oninput: move |e| new_ingredient.name = e.value(),
                                    }
                                }
                            }
                            button {
                                class: "button buttonBg3",
                                type: "button",
                                onclick: move |_| ingredients.push(NewIngredient::default()),
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
    }
}
