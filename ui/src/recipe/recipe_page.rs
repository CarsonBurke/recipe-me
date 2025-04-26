use dioxus::prelude::*;

#[component]
pub fn RecipePage(id: i32) -> Element {
    rsx! {
        h1 { "Recipe Page with id {id}" }
        div {
            class: "row bg3 paddingMedium",
            "Value"
        }
    }
}