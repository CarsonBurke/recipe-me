use std::f32::EPSILON;

use dioxus::{html::u, prelude::*};

use crate::{components::recipe::preview::{RecipePreview, Selected}, server::collection::{get_collection, my_collection_recipes}};

#[component]
pub fn CollectionLocal(id: ReadOnlySignal<i32>) -> Element {
    let collection = use_resource(move || {
        let cloned_id = id();
        async move { get_collection(cloned_id).await }
    })
    .suspend()?;

    let recipes = use_resource(move || {
        let cloned_id = id();
        async move { my_collection_recipes(cloned_id).await }
    }).suspend()?;

    rsx! {
        main {
            class: "main",
            if let Some(collection) = collection() {
                section {
                    class: "section column gapLarge",
                    div {
                        h1 {
                            class: "textLarge",
                            {collection.collection_name}
                        }
                        p {
                            class: "textSmall",
                            {collection.description.unwrap_or("".to_string())}
                        }
                    }
                    div {
                        class: "row flexWrap gapSmall centerRow",
                        for recipe in recipes() {
                            RecipePreview {
                                id: recipe.id,
                                name: recipe.name,
                                summary: recipe.summary,
                                source: recipe.source,
                                rating: (recipe.total_rating as f32) / (recipe.ratings as f32 + EPSILON),
                                selected: Selected::NoSelect,
                                public: false,
                            }
                        }       
                    }
                }
            }
            else {
                h1 { "Could not find collection" }
            }
        }
    }
}
