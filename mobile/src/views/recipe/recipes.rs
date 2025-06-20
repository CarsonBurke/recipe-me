use api::{get_filtered_recipes, get_recipes, FilteredRecipesParams};
use dioxus::{logger::tracing::info, prelude::*};
use dioxus_free_icons::icons::ld_icons;
use serde::{Deserialize, Serialize};

use crate::{
    components::recipe::{filtered_local, filtered_public},
    Route,
};

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Query {
    pub cuisine_id: Option<i32>,
    pub diet_id: Option<i32>,
    pub ingredient_id: Option<i32>,
    pub meal_id: Option<i32>,
    pub limit: Option<u64>,
    pub recipe_select: bool,
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
            cuisine_id: res.cuisine_id,
            ingredient_id: res.ingredient_id,
            meal_id: res.meal_id,
            diet_id: res.diet_id,
            limit: res.limit,
            recipe_select: res.recipe_select,
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
pub fn Recipes(query: ReadOnlySignal<Query>) -> Element {
    let query_read = query();

    rsx! {
        main {
            class: "main column gapMedium",
            section {
                class: "section column",
                div {
                    class: "row gapMedium centerColumn spaceBetween",
                    if query_read.public {
                        h1 { class: "textLarge", "Public recipes" }
                    }
                    else {
                        h1 { class: "textLarge", "My recipes" }
                    }
                    div {
                        class: "row flexWrap gapSmall centerRow",
                        if query_read.public {
                            Link {
                                class: "buttonSmall buttonBg2",
                                to: Route::Recipes {
                                    query: Query {
                                        public: false,
                                        ..Default::default()
                                    }
                                },
                                dioxus_free_icons::Icon { icon: ld_icons::LdBook }
                                "My recipes"
                            }
                        }
                        else {
                            
                            Link {
                                class: "buttonSmall buttonBg2",
                                to: Route::Recipes {
                                    query: Query {
                                        public: true,
                                        recipe_select: true,
                                        ..Default::default()
                                    }
                                },
                                dioxus_free_icons::Icon { icon: ld_icons::LdCloudDownload }
                                "Get recipe"
                            }
                        }
                        Link {
                            class: "buttonSmall buttonBg2",
                            to: Route::NewRecipeView {},
                            dioxus_free_icons::Icon { icon: ld_icons::LdPlus }
                            "New recipe"
                        }
                    }
                }
                div {
                    class: "row gapMedium centerRow flexWrap",
                    if query_read.public {
                        filtered_public::FilteredRecipes {
                            cuisine_id: query_read.cuisine_id,
                            diet_id: query_read.diet_id,
                            ingredient_id: query_read.ingredient_id,
                            meal_id: query_read.meal_id,
                            limit: query_read.limit,
                            recipe_select: query_read.recipe_select,
                        }
                    }
                    else {
                        filtered_local::FilteredRecipes {
                            cuisine_id: query_read.cuisine_id,
                            diet_id: query_read.diet_id,
                            ingredient_id: query_read.ingredient_id,
                            meal_id: query_read.meal_id,
                            limit: query_read.limit,
                            recipe_select: query_read.recipe_select,
                        }
                    }
                }
            }
        }
    }
}
