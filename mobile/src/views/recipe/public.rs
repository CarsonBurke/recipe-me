use dioxus::{logger::tracing::info, prelude::*};

use crate::components::recipe::recipe::Recipe;

#[component]
pub fn RecipePage(id: ReadOnlySignal<i32>) -> Element {
    rsx! {
        Recipe { 
            id,
            is_local: false
        }
    }
}
