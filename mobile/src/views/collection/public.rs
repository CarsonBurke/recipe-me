use std::f32::EPSILON;

use dioxus::{html::u, prelude::*};

use crate::{
    components::recipe::preview::{RecipePreview, Selected},
    server::collection::{get_collection, my_collection_recipes},
};

#[component]
pub fn CollectionPublic(id: ReadOnlySignal<i32>) -> Element {
    let collection = use_resource(move || {
        let cloned_id = id();
        async move { api::get_collection(cloned_id).await }
    })
    .suspend()?;

    let recipes = use_resource(move || {
        let cloned_id = id();
        async move {
            api::get_filtered_recipes(api::FilteredRecipesParams {
                collection_id: Some(cloned_id),
                ..Default::default()
            })
            .await
        }
    })
    .suspend()?;

    rsx! {
        main {
            class: "main",
            if let Ok(collection) = collection() {
                section {
                    class: "section column gapLarge",
                    div {
                        h1 {
                            class: "textLarge",
                            {collection.collection_name}
                        }
                        p {
                            class: "textSmall",
                            {"no description".to_string()/* collection.description.unwrap_or("".to_string()) */}
                        }
                    }
                    div {
                        class: "row flexWrap gapSmall centerRow",
                        if let Ok(recipes) = recipes() {
                            for recipe in recipes {
                                RecipePreview {
                                    id: recipe.id,
                                    name: recipe.name,
                                    summary: recipe.summary,
                                    source: recipe.source,
                                    total_rating: recipe.total_rating,
                                    ratings: recipe.ratings,
                                    selected: Selected::NoSelect,
                                    public: false,
                                }
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
