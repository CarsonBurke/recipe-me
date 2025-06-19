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
    let collections = use_resource(|| async move { get_my_collections().await });

    rsx! {
        if let Some(collections) = collections() {
            for collection in collections.iter() {
                CollectionPreview {
                    id: collection.id,
                    public: true,
                    name: collection.collection_name.clone(),
                    description: collection.description.clone(),
                }
            }
        }
    }
}
