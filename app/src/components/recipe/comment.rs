use dioxus::prelude::*;

use crate::components::RatingStatic;

#[component]
pub fn RecipeComment(user_id: i32, username: String, comment: String, rating: f32) -> Element {
    rsx! {
        div { 
            class: "recipeComment column gapSmall",
            div {
                class: "row gapMedium",
                h2 {
                    class: "textMedium",
                    {username}
                }
                RatingStatic { 
                    rating
                }
            }
            p {
                class: "textSmall",
                "{comment}"
            }
        }
    }
}