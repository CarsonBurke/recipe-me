use dioxus::prelude::*;

use crate::{components::collection::preview::CollectionPreview, server::collection::get_my_collections};

#[component]
pub fn CollectionPreviews(add_recipe: Option<i32>) -> Element {
    let collections = use_resource(|| async move { get_my_collections().await });

    rsx! {
        if let Some(collections) = collections() {
            for collection in collections.iter() {
                CollectionPreview {
                    id: collection.id,
                    name: collection.collection_name.clone(),
                }
            }
        }
    }
}