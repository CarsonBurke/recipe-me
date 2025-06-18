use dioxus::prelude::*;

use crate::components::recipe::recipe::Recipe;

#[component]
pub fn RecipeLocal(id: ReadOnlySignal<i32>) -> Element {
    rsx! {
        Recipe {
            id,
            is_public: false
        }
    }
}
