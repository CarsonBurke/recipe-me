use dioxus::prelude::*;

use crate::recipe::recipe_page::RecipePage;

#[component]
pub fn Recipe(id: i32) -> Element {
    rsx! {
        "Recipe"
        RecipePage { id }
    }
}