use dioxus::prelude::*;

#[component]
pub fn RecipeComments(recipe_id: i32) -> Element {
    rsx! {
        div { 
            class: "recipe-comments",
            "Comments"
        }
    }
}