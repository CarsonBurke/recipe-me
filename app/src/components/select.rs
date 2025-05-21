use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Params {
    title: String,
    options: Vec<String>,
}

#[component]
pub fn Select(params: Params) -> Element {
    rsx! {
        select {
            class: "select button buttonBg1",
            p { class: "textSmall", {params.title} }
            for option in params.options {
                option { "{option}" }
            }
        }
    }
}
