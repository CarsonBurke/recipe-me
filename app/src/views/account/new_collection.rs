use dioxus::prelude::*;

#[component]
pub fn NewCollection() -> Element {
    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
            }
        }
    }
}