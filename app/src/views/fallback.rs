use dioxus::{html::nav, prelude::*};

use crate::Route;

#[component]
pub fn Fallback(route: Vec<String>) -> Element {
    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    class: "column gapMedium round bg2 paddingMedium",
                    h1 {
                        class: "textLarge",
                        "This is not the page you are looking for."
                    }
                    div {
                        class: "row gapMedium",
                        button {
                            class: "buttonBg3 button",
                            onclick: move |_| {
                                let navigator = navigator();
                                navigator.go_back();
                            },
                            "Back"
                        }
                        Link {
                            class: "buttonBg3 button",
                            to: Route::Home {},
                            "Go Home"
                        }
                    }
                }
            }
        },
    }
}