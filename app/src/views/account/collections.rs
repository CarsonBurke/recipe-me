use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::Route;

#[component]
pub fn AccountCollections() -> Element {
    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    class: "row gapMedium centerColumn",
                    h1 { class: "textLarge", "My collections" }
                    Link {
                        class: "button buttonBg2",
                        to: Route::NewCollection {},
                        dioxus_free_icons::Icon { icon: ld_icons::LdPlus }
                        "New collection"
                    }   
                }
                div {
                    class: "column gapMedium centerColumn",
                    p { class: "textMedium", "You have no collections" }
                }
            }
        }
    }
}
