use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::{components::collection::{collections::CollectionPreviews, preview::CollectionPreview}, server::collection::get_my_collections, Route};

#[component]
pub fn Collections() -> Element {
    let collections = use_resource(|| async move { get_my_collections().await });

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    class: "row gapMedium centerColumn spaceBetween",
                    h1 { class: "textLarge", "My collections" }
                    Link {
                        class: "button buttonBg2",
                        to: Route::NewCollection {},
                        dioxus_free_icons::Icon { icon: ld_icons::LdPlus }
                        "New collection"
                    }
                }
                div {
                    class: "column gapMedium centerColumn",
                    div {
                        class: "row flexWrap gapSmall centerRow",
                        CollectionPreviews {  }
                    }
                }
            }
        }
    }
}
