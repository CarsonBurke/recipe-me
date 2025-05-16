use dioxus::{html::textarea, prelude::*};
use dioxus_free_icons::icons::ld_icons;

#[component]
pub fn NewRecipe() -> Element {
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());
    let ingredients = use_signal::<Vec<i32>>(|| vec![]);
    let instructions = use_signal(|| "".to_string());

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
                    },
                    class: "column gapLarge paddingLarge round bg2 centerColumn",
                    h1 {class: "textLarge", "New recipe" },
                    div {
                        class: "column gapMedium centerColumn",
                        input {
                            placeholder: "Recipe name",
                            type: "text",
                            class: "input bg3 borderBg4"
                        }
                        textarea { class: "input bg3 borderBg4", placeholder: "Recipe description" }
                        div {
                            class: "column gapSmall",
                            p { class: "textMedium", "Ingredients" }
                            div {
                                class: "row gapSmall centerColumn",
                                input {
                                    placeholder: "2",
                                    type: "number",
                                    min: 0,
                                    max: 1000,
                                    class: "bg3 borderBg4 inputShort",
                                    size: "3",
                                }
                                input {
                                    placeholder: "cups",
                                    type: "text",
                                    class: "bg3 borderBg4 inputShort",
                                    size: "3",
                                }
                                p { class: "textMedium", "of" }
                                input {
                                    placeholder: "ground beef",
                                    type: "text",
                                    class: "input bg3 borderBg4",
                                    size: "10",
                                }
                            }
                            
                        }
                        textarea { class: "input bg3 borderBg4", placeholder: "Recipe instructions" }
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
