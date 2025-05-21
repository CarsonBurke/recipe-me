use dioxus::prelude::*;

#[component]
pub fn RecipePreview(name: String) -> Element {
    rsx! {
        div {
            class: "row bg3 paddingMedium",
            {name}
        }
    }
}