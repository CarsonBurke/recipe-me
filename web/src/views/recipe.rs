use dioxus::prelude::*;
use ui::recipe::recipe_page::RecipePage;

#[component]
pub fn Recipe(id: i32) -> Element {
    rsx! {
        "Recipe"
        RecipePage { id }
    }
}