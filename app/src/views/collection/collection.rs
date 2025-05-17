use api::get_collection;
use dioxus::prelude::*;

use crate::components::filtered_recipes::FilteredRecipes;

#[component]
pub fn CollectionPage(id: ReadOnlySignal<i32>) -> Element {

    let collection = use_server_future(move || {
        async move { get_collection(id()).await.unwrap() }
    })?;
    let collection_read = collection.read();
    let collection_ref = collection_read.as_ref().unwrap();

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                h1 {
                    class: "textLarge",
                    {collection_ref.collection_name.clone()},
                }
                FilteredRecipes { 
                    collection_id: Some(id())
                }
            }
        }
    }
}