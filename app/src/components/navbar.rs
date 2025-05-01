use dioxus::prelude::*;

use crate::Route;

const CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar(children: Element) -> Element {
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
                {children}
            }
        }
    }
}
