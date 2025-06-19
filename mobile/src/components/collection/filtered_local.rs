use api::user_actions::{get_all_collections, get_collections};
use dioxus::prelude::*;

use crate::{
    components::collection::preview::CollectionPreview, server::collection::get_my_collections,
};

pub struct AddRecipe {
    pub recipe_id: i32,
    pub collection: bool,
}

#[component]
pub fn CollectionPreviews(add_recipe: Option<Signal<AddRecipe>>) -> Element {
    let collections = use_resource(|| async move { get_all_collections(0, 50).await.unwrap() });

    rsx! {
        if let Some(collections) = collections() {
            for collection in collections.iter() {
                CollectionPreview {
                    id: collection.id,
                    public: false,
                    name: collection.collection_name.clone(),
                    description: "no description".to_string(),
                }
            }
        }
    }
}
