use dioxus::{logger::tracing::info, prelude::*};

use crate::components;

#[component]
pub fn RecipePublic(id: ReadOnlySignal<i32>) -> Element {
    rsx! {
        components::recipe::public::RecipePublic { 
            id,
        }
    }
}
