use dioxus::prelude::*;

use crate::{utils::logout, Route};

#[component]
pub fn Account() -> Element {
    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapMedium",
                div {
                    class: "column round gapMedium bg2 paddingMedium",
                    h1 {
                        class: "textLarge",
                        "Account"
                    }
                    button {
                        class: "buttonBg3 button",
                        onclick: move |_| {
                            logout();
                            navigator().push(Route::Home {});
                        },
                        "Logout"
                    }
                }
            }
        }
    }
}