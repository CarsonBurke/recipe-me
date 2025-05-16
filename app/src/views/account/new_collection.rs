use dioxus::prelude::*;

#[component]
pub fn NewCollection() -> Element {

    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                form {
                    class: "column gapLarge paddingLarge round bg2 centerColumn",
                    h1 {class: "textLarge", "New collection" },
                    input { class: "input bg3 borderBg4", placeholder: "Collection name" }
                    textarea { class: "input bg3 borderBg4", placeholder: "Collection description" }
                    button { class: "button buttonBg3", "Create collection" }
                }
            }
        }
    }
}