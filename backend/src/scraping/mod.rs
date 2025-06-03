use std::{error::Error, fmt::Display};

use all_recipes::scrape_all_recipes;
use fantoccini::{ClientBuilder, Locator, error::NewSessionError, wd::WebDriverCompatibleCommand};
use futures::StreamExt;
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    prelude::Decimal,
};
use tokio::{fs, time::error::Elapsed};

use crate::{
    db::db_conn,
    entities::{ingredient_name, recipe, recipe_ingredient},
    scraping::bbc_food::scrape_bbc_food,
};

mod all_recipes;
mod bbc_food;
mod generate;

/// Listen to either the address of (run) `chromedriver` or `geckodriver`
static DRIVER_ADDRESS: &str = "http://localhost:36835";

#[derive(Debug)]
pub enum ScrapeError {
    FailedGeneral,
    FantocciniCmdError(fantoccini::error::CmdError),
    FantocciniNewSessionError(fantoccini::error::NewSessionError),
}

impl Error for ScrapeError {}

impl Display for ScrapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to scrape")
    }
}

impl From<fantoccini::error::CmdError> for ScrapeError {
    fn from(err: fantoccini::error::CmdError) -> Self {
        ScrapeError::FantocciniCmdError(err)
    }
}

impl From<NewSessionError> for ScrapeError {
    fn from(err: NewSessionError) -> Self {
        ScrapeError::FantocciniNewSessionError(err)
    }
}

pub async fn scrape() {
    let sites = vec![
        /* Site::new(
            "bbc_food".to_string(),
            "https://www.bbc.co.uk/food/".to_string(),
        ), */
        Site::new(
            "all_recipes".to_string(),
            "https://www.allrecipes.com/".to_string(),
        ),
    ];

    for site in sites {
        match site.name.as_str() {
            "bbc_food" => scrape_bbc_food(&site).await.unwrap(),
            "all_recipes" => scrape_all_recipes(&site).await.unwrap(),
            _ => panic!("Unknown site"),
        };
    }
}

#[derive(Debug)]
pub struct ScrapedIngredient {
    name: String,
    description: String,
    amount: f32,
}

impl ScrapedIngredient {
    pub async fn find_in_db_by_name(&self, db_conn: &DatabaseConnection) -> Option<i32> {
        let ingredient = ingredient_name::Entity::find()
            .filter(ingredient_name::Column::Name.eq(self.name.clone()))
            .one(db_conn)
            .await
            .unwrap();

        if let Some(ingredient) = ingredient {
            println!("Found ingredient with id: {}", ingredient.id);
            return Some(ingredient.id);
        }

        None
    }
}

#[derive(Debug)]
pub struct ScrapedRecipe {
    title: String,
    ingredients: Vec<ScrapedIngredient>,
    description: String,
    instructions: String,
    ratings_count: u32,
    average_rating: f32,
    time: Option<String>,
    url: String,
    image_url: Option<String>,
}

impl ScrapedRecipe {
    pub async fn try_write(&self, db_conn: &DatabaseConnection) {
        println!("Trying to write recipe");
        let instance = recipe::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(self.title.clone()),
            description: ActiveValue::Set(self.description.clone()),
            source: ActiveValue::Set(Some(self.url.clone())),
            summary: ActiveValue::Set({
                let mut cloned_description = self.description.clone();
                cloned_description.shrink_to(100);
                cloned_description
            }),
            instructions: ActiveValue::Set(self.instructions.clone()),
            views: ActiveValue::Set(Some(0)),
            ratings: ActiveValue::Set(self.ratings_count as i32),
            total_rating: ActiveValue::Set(
                (self.ratings_count as f32 * self.average_rating) as i32,
            ),
            author_id: ActiveValue::NotSet,
            public: ActiveValue::Set(Some(true)),
            image_url: ActiveValue::Set(self.image_url.clone()),
        };

        let result = instance.insert(db_conn).await;

        if let Ok(result) = result {
            println!("inserted recipe with id {}", result.id);

            self.try_write_ingredients(db_conn, result.id).await;
        } else {
            println!("failed to insert recipe");
        }
    }

    pub async fn try_write_ingredients(&self, db_conn: &DatabaseConnection, recipe_id: i32) {
        for ingredient in &self.ingredients {
            let existing_ingredient_id = ingredient.find_in_db_by_name(db_conn).await;

            let ingredient_id = if let Some(existing_ingredient_id) = existing_ingredient_id {
                existing_ingredient_id
            } else {
                let ingredient_name = ingredient_name::ActiveModel {
                    id: ActiveValue::NotSet,
                    name: ActiveValue::Set(ingredient.name.clone()),
                    affiliate_link: ActiveValue::NotSet,
                };
                let Ok(ingredient_name_result) = ingredient_name.insert(db_conn).await else {
                    continue;
                };

                ingredient_name_result.id
            };

            let recipe_ingredient = recipe_ingredient::ActiveModel {
                ingredient_id: ActiveValue::Set(ingredient_id),
                description: ActiveValue::Set(ingredient.description.clone()),
                amount: ActiveValue::Set(
                    Decimal::from_f32_retain(ingredient.amount).expect("Invalid amount"),
                ),
                recipe_id: ActiveValue::Set(recipe_id),
            };

            if let Ok(recipe_ingredient) = recipe_ingredient.insert(db_conn).await {
                println!(
                    "inserted recipe ingredient with id {}",
                    recipe_ingredient.ingredient_id
                );
            } else {
                println!("failed to insert recipe ingredient");
            }
        }
    }
}

async fn is_recipe_url_in_db(db_conn: &DatabaseConnection, url: &String) -> bool {
    recipe::Entity::find()
        .filter(recipe::Column::Source.eq(url))
        .one(db_conn)
        .await
        .unwrap()
        .is_some()
}

pub struct Site {
    name: String,
    // Where people can find the recipes
    reference_url: String,
}

impl Site {
    fn new(name: String, reference_url: String) -> Self {
        Self {
            name,
            reference_url,
        }
    }

    pub fn recipe_hrefs_path(&self) -> String {
        format!("./recipe_hrefs/{}.json", self.name)
    }

    pub async fn get_cached_hrefs(&self) -> Option<Vec<String>> {
        if let Ok(content) = fs::read(self.recipe_hrefs_path()).await {
            let hrefs: Vec<String> = serde_json::from_slice(&content).unwrap();
            println!("Loaded hrefs from file of len {}", hrefs.len());
            return Some(hrefs);
        };

        None
    }

    pub async fn write_relative_hrefs(&self, hrefs: &[String]) {
        fs::write(
            self.recipe_hrefs_path(),
            serde_json::to_string(hrefs).unwrap(),
        )
        .await
        .unwrap();
    }
}

// Would probably be a good idea to use a trait for Site stuff so less templating is required and things can be more streamlined
