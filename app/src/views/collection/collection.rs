use dioxus::prelude::*;

#[component]
pub fn CollectionPage(id: ReadOnlySignal<i32>) -> Element {
    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
            }
        }
    }
}