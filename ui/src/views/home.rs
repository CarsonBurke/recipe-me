use dioxus::prelude::*;

use crate::{Echo, Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}
