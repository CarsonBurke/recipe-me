use dioxus::prelude::*;

use crate::{components::collection::preview::CollectionPreview, server::collection::get_my_collections};

#[component]
pub fn CollectionPreviews(add_recipe: Option<i32>, public: bool) -> Element {
    let collections = use_resource(|| async move { get_my_collections().await });

    rsx! {
        if let Some(collections) = collections() {
            for collection in collections.iter() {
                CollectionPreview {
                    id: collection.id,
                    public,
                    name: collection.collection_name.clone(),
                    description: collection.description.clone(),
                }
            }
        }
    }
}