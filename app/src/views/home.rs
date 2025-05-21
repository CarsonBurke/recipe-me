use dioxus::prelude::*;

use crate::components::{
    Echo, Hero,
    filtered_recipes::{self, FilteredRecipes},
};

#[component]
pub fn Home() -> Element {
    rsx! {
        main {
            class: "column gapLarge",
            Hero {}
            section {
                class: "section column gapLarge",
                h1 {
                    class: "textLarge",
                    "Popular recipes"
                }
                div {
                    class: "row gapMedium centerRow flexWrap",
                    FilteredRecipes { public: Some(true) }
                }
            }
        }
    }
}
