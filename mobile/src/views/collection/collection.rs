use dioxus::prelude::*;

#[component]
pub fn Collection(id: ReadOnlySignal<i32>) -> Element {
    rsx! {
        h1 { "Collection {id}" }
        h1 { "Other collections" }
    }
}