use dioxus::prelude::*;

use crate::components::{self};

#[component]
pub fn RecipeLocal(id: ReadOnlySignal<i32>) -> Element {
    rsx! {
        components::recipe::local::RecipeLocal {
            id,
        }
    }
}
