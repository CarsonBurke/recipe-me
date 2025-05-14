use dioxus::prelude::*;

use crate::{views::recipe::recipes, Route};

const CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        nav {
            id: "navbar",
            class: "navbar paddingSectionSide row bg3 gapSmall",
            Link {
                to: Route::Home {},
                h1 {
                    class: "textLarge button buttonBg3",
                    "Recipe Me"
                }
            }
            div {
                class: "row centerColumn gapMedium",
                Link {
                    class: "buttonBg3 button",
                    to: Route::Home {},
                    "Home"
                }
                Link {
                    class: "buttonBg3 button",
                    to: Route::Recipes { query: recipes::Query::default() },
                    "Recipes"
                }
                Link {
                    class: "buttonBg3 button",
                    to: Route::Login {  },
                    "Login"
                }
                Link {
                    class: "buttonBg3 button",
                    to: Route::Signup {  },
                    "Create an account"
                }
            }
        }
    }
}
