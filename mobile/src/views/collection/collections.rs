use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;
use serde::{Deserialize, Serialize};

use crate::{components::collection::{self, filtered_public, preview::CollectionPreview}, server::collection::get_my_collections, Route};

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Query {
    pub public: bool,
}

impl From<&str> for Query {
    fn from(query: &str) -> Self {
        let parsed = serde_json::from_str::<Query>(query);

        let Ok(res) = parsed else {
            return Self {
                ..Default::default()
            };
        };

        Self {
            public: res.public,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = serde_json::to_string(self).unwrap();
        write!(f, "{}", str)
    }
}

#[component]
pub fn Collections(query: ReadOnlySignal<Query>) -> Element {

    let query_read = query();

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    class: "row gapMedium centerColumn spaceBetween",
                    if query_read.public {
                        h1 { class: "textLarge", "Browse collections" }
                    }
                    else {
                        h1 { class: "textLarge", "My collections" }
                    }
                    div {
                        class: "row gapSmall",
                        if query_read.public {
                            Link {
                                class: "buttonSmall buttonBg2",
                                to: Route::Collections { query: Query { public: false } },
                                dioxus_free_icons::Icon { icon: ld_icons::LdBook }
                                "My collections"
                            }
                        }
                        else {
                            Link {
                                class: "buttonSmall buttonBg2",
                                to: Route::Collections { query: Query { public: true } },
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
                        if query_read.public {
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
