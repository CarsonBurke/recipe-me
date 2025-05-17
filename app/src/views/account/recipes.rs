use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;
use serde::{Deserialize, Serialize};

use crate::{components::filtered_recipes::FilteredRecipes, Route, LOGIN_TOKEN_GLOBAL};

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Query {
    pub cuisine_id: Option<i32>,
    pub diet_id: Option<i32>,
    pub ingredient_id: Option<i32>, 
    pub meal_id: Option<i32>,
    pub limit: Option<u64>,
}

impl From<&str> for Query {
    fn from(query: &str) -> Self {

        let parsed = serde_json::from_str::<Query>(query);

        let Ok(res) = parsed else {
            return Self {
                ..Default::default()
            }
        };

        Self {
            cuisine_id: res.cuisine_id,
            ingredient_id: res.ingredient_id,
            meal_id: res.meal_id,
            diet_id: res.diet_id,
            limit: res.limit,
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
pub fn AccountRecipes(query: Query) -> Element {

    let login_token = LOGIN_TOKEN_GLOBAL();

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    class: "row gapMedium centerColumn",
                    h1 { class: "textLarge", "My recipes" }
                    Link {
                        class: "button buttonBg2",
                        to: Route::NewRecipe {},
                        dioxus_free_icons::Icon { icon: ld_icons::LdPlus }
                        "New recipe"
                    }
                }
                div {
                    class: "column gapMedium centerColumn",
                    if let Some(login_token) = login_token {
                        FilteredRecipes { author_id: login_token.user_id }
                    }
                    else {
                        p { class: "textMedium", "You have no recipes" }
                    }
                }
            }
        }
    }
}