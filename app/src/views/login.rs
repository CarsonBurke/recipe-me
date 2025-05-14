use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        div {
            class: "login",
            h1 { "Login" }
        }
    }
}