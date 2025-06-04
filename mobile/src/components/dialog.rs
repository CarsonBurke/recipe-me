use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/dialog.css");

#[component]
pub fn Dialog(children: Children) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        div {
            class: "dialog column gapMedium paddingLarge bg2 hidden",
            children
        }
    }
}

pub fn toggle_dialog_onclick(id: String) {
    let dialog = document::get_element_by_id(&id).unwrap();
    dialog.set_class("hidden", false);
}