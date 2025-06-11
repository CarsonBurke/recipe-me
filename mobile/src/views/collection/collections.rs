use api::user_actions::get_collections;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::{Route};

#[component]
pub fn Collections() -> Element {

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    class: "row gapMedium centerColumn spaceBetween",
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
                    "Collections"
                }
            }
        }
    }
}
