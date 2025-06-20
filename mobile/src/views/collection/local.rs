use std::f32::EPSILON;

use dioxus::{html::u, prelude::*};
use dioxus_free_icons::icons::ld_icons;

use crate::{
    components::{
        dialog::DialogWrapper,
        recipe::preview::{RecipePreview, Selected},
    },
    server::{
        self,
        collection::{get_collection, my_collection_recipes},
    },
};

#[component]
pub fn CollectionLocal(id: ReadOnlySignal<i32>) -> Element {
    let collection = use_resource(move || {
        let cloned_id = id();
        async move { get_collection(cloned_id).await }
    })
    .suspend()?;

    let Some(collection_read) = collection() else {
        return rsx! {
            h1 { class: "textLarge", "Collection not found" }
        };
    };

    let recipes = use_resource(move || {
        let cloned_id = id();
        async move { my_collection_recipes(cloned_id).await }
    })
    .suspend()?;

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    class: "row gapMedium centerColumn spaceBetween",
                    div {
                        h1 {
                            class: "textLarge",
                            {collection_read.collection_name}
                        }
                        p {
                            class: "textSmall",
                            {collection_read.description.unwrap_or("".to_string())}
                        }
                    }
                    div {
                        class: "row gapSmall",
                        DialogWrapper {
                            header: rsx! {
                                h1 { class: "textLarge", "Delete recipe" }
                            },
                            button: rsx! {
                                button {
                                    class: "button buttonBg2 textXSmall",
                                    onclick: move |_| {
                                        println!("Delete collection");
                                    },
                                    dioxus_free_icons::Icon { icon: ld_icons::LdTrash }
                                }
                            },
                            dialog: rsx! {
                                div {
                                    class: "column gapMedium round centerColumn",
                                    p { class: "textSmall", "Are you sure you want to delete this recipe?" },
                                    button {
                                        class: "button buttonBg3 textNegative widthFit",
                                        onclick: move |_| async move {

                                            server::collection::delete_collection(id()).await.unwrap();

                                            navigator().go_back();
                                        },
                                        "Delete"
                                    }
                                }
                            }
                        }
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
}
