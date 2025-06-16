use dioxus::{logger::tracing::info, prelude::*};

use crate::components::recipe::comp::RecipeComp;

#[component]
pub fn RecipePage(id: ReadOnlySignal<i32>) -> Element {
    RecipeComp { 
        id,
        local: false
    }
}
