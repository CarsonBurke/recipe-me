use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

const CSS: Asset = asset!("/assets/styling/dialog.css");

#[component]
pub fn DialogWrapper(button: Element, header: Element, dialog: Element) -> Element {
    let mut hidden = use_signal(|| true);
    let hidden_class = if hidden() { "hidden" } else { "" };

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        button {
            /* onclick: move |event| {
                let target = event.target();

                println!("target: {target}");
            }, */
            class: "bgTransparent",
            div {
                onclick: move |_| hidden.set(!hidden()),
                {button}
            }
            div {
                class: format!("dialog column gapLarge round paddingLarge bg2 {hidden_class}"),
                div {
                    class: "row gapLarge dialogHeader centerRow",
                    {header}
                    button {
                        class: "dialogClose button buttonBg2 centerColumn centerRow",
                        onclick: move |_| hidden.set(true),
                        dioxus_free_icons::Icon { icon: ld_icons::LdX }
                    }
                }
                {dialog}
            }
        }
    }
}

/* pub fn toggle_dialog_onclick(id: String) {
    let dialog = document::get_element_by_id(&id).unwrap();
    dialog.set_class("hidden", false);
} */
