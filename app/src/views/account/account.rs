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
                    class: "column round gapMedium bg2 paddingLarge centerColumn",
                    h1 {
                        class: "textLarge textCenter",
                        "Account"
                    }
                    button {
                        class: "buttonBg3 button widthFit",
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