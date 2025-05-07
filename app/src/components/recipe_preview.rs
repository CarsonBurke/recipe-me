use dioxus::prelude::*;

use crate::{components::RatingStatic, Route};

const CSS: Asset = asset!("/assets/styling/recipe_preview.css");

#[component]
pub fn RecipePreview(id: i32, name: String, summary: String, source: Option<String>, rating: f32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        Link {
            class: "recipe_preview column round borderBg2 button buttonBg1 gapMedium",
            to: Route::RecipePage { id, },
            div {
                class: "column gapSmall paddingSmall",
                div {
                    class: "row gapMedium",
                    h2 {
                        class: "textMedium",
                        "{name}"
                    }
                    RatingStatic { 
                        rating
                    }
                }
                p {
                    "{summary}",
                }
            }
            div {
                class: "recipe_image round"
            }
        }
    }
}
