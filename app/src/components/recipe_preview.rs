use dioxus::prelude::*;

use crate::Route;

const CSS: Asset = asset!("/assets/styling/recipe_preview.css");

#[component]
pub fn RecipePreview(id: i32, name: String, summary: String, source: Option<String>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        Link {
            class: "recipe_preview column paddingMedium round borderBg2 button buttonBg1 gapMedium",
            to: Route::RecipePage { id, },
            div {
                class: "column gapSmall",
                h2 {
                    class: "textMedium",
                    "{name}"
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
