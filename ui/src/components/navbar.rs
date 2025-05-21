use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        nav {
            id: "navbar",
            class: "navbar paddingSectionSide row bg3 gapSmall",
            header {
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
