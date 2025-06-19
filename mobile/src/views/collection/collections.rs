use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::{components::collection::{self, filtered_public, preview::CollectionPreview}, server::collection::get_my_collections, Route};

#[component]
pub fn Collections(public: bool) -> Element {

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    class: "row gapMedium centerColumn spaceBetween",
                    if public {
                        h1 { class: "textLarge", "Browse collections" }
                    }
                    else {
                        h1 { class: "textLarge", "My collections" }
                    }
                    div {
                        class: "row gapSmall",
                        if public {
                            Link {
                                class: "buttonSmall buttonBg2",
                                to: Route::Collections { public: false, },
                                dioxus_free_icons::Icon { icon: ld_icons::LdGlobe }
                                "My collections"
                            }
                        }
                        else {
                            Link {
                                class: "buttonSmall buttonBg2",
                                to: Route::Collections { public: true, },
                                dioxus_free_icons::Icon { icon: ld_icons::LdGlobe }
                                "Browse collections"
                            }
                        }
                        Link {
                            class: "buttonSmall buttonBg2",
                            to: Route::NewCollection {},
                            dioxus_free_icons::Icon { icon: ld_icons::LdPlus }
                            "New collection"
                        }
                    }
                }
                div {
                    class: "column gapMedium centerColumn",
                    div {
                        class: "row flexWrap gapSmall centerRow",
                        if public {
                            filtered_public::CollectionPreviews { }
                        }
                        else {
                            collection::filtered_local::CollectionPreviews { }
                        }
                    }
                }
            }
        }
    }
}
