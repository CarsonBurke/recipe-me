use api::user_actions::get_collections;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::{Route, LOGIN_TOKEN_GLOBAL};

#[component]
pub fn AccountCollections() -> Element {

    let login_token = LOGIN_TOKEN_GLOBAL();

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    class: "row gapMedium centerColumn",
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
                    {
                        if let Some(login_token) = login_token {

                            let collections = use_server_future(move || {
                                let login_token_cloned = login_token.clone();

                                async move { get_collections(login_token_cloned).await.unwrap() }
                            })?;

                            let collections_read = collections.read();
                            let collections_ref = collections_read.as_ref().unwrap();

                            if collections_ref.is_empty() {
                                return rsx! {
                                    p { class: "textMedium", "You have no collections" }
                                };
                            }

                            rsx! {
                                for collection in collections_ref.iter() {
                                    Link {
                                        class: "button buttonBg2",
                                        to: Route::CollectionPage { id: collection.id },
                                        "{collection.collection_name}"
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                p { class: "textMedium", "You are not logged in" }
                            }
                        }
                    }
                    
                }
            }
        }
    }
}
