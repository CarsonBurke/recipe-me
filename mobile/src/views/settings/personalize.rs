use dioxus::{logger::tracing::info, prelude::*};
use dioxus_sdk::storage::{use_persistent, use_synced_storage, LocalStorage};

use crate::{constants, Theme, THEME_GLOBAL};

#[component]
pub fn Personalize() -> Element {
    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    h1 { "Preferences" }
                    div {
                        class: "column gapSmall bg2 round paddingLarge width100",
                        select {
                            class: "button buttonBg2",
                            oninput: move |e| {

                                let value = e.data.value();

                                let mut cached_theme = use_persistent::<Theme>(constants::THEME.to_string(), || {
                                    Theme::default()
                                });
                                cached_theme.set(Theme::from(value.clone()));
                                println!("set value {}", cached_theme);

                                *THEME_GLOBAL.write() = cached_theme();
                            },
                            for theme in enum_iterator::all::<Theme>() {
                                option {
                                    value: theme.file_name(),
                                    "{theme}"
                                }
                            }
                        }
                        hr { class: "bg3"}
                    }
                }
            }
        }
    }
}
