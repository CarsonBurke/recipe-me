use dioxus::prelude::*;

use crate::components::{
    Echo, Hero,
    filtered_recipes::{self, FilteredRecipes},
};

#[component]
pub fn Home() -> Element {
    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapMedium",
                h1 {
                    class: "textXLarge",
                    "Recipes"
                }
                FilteredRecipes {
                    params: filtered_recipes::Params::default()
                }
            }
        }
    }
}
