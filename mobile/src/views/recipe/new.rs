use api::user_actions::{self, NewIngredient};
use dioxus::{html::textarea, prelude::*};
use dioxus_free_icons::icons::ld_icons;

use crate::{components::recipe::new::NewRecipe, Route};

#[component]
pub fn NewRecipeView() -> Element {

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                NewRecipe {  }
            }
        }
    }
}
