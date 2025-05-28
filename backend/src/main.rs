use std::{error::Error, fmt::Display};

use axum::{http::StatusCode, routing::{get, post}, Json, Router};
use db::{db_conn, sample_data::create_sample_data};
use entities::{recipe, recipe_cuisine, recipe_diet, recipe_ingredient, recipe_meal};
use scraping::scrape_bbc_food;
use sea_orm::{sea_query::Query, ColumnTrait, Condition, EntityTrait, QueryFilter, QuerySelect, QueryTrait};
use serde::{Deserialize, Serialize};

mod db;
mod secrets;
mod entities;
mod scraping;

#[tokio::main]
async fn main() {
    /* tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(launch_server()); */
    launch_server().await;
}

async fn launch_server() {

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/filtered_recipes", post(get_filtered_recipes));

    tokio::spawn(async {
        // Can spawn an async task here
        println!("Spawning runner thread");
        scrape_bbc_food().await.unwrap();

        println!("Scraping complete");
    });

    println!("Listening and serving server");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

/*     // Connect to dioxus' logging infrastructure
    
    let db = db_conn().await.unwrap();

    create_sample_data().await.unwrap(); */
}

async fn root() -> &'static str {
    "Root content"
}

#[derive(Debug)]
pub enum ServerError {
    ServerError(String),
}

impl Error for ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Server error: {}", self)
    }
}

#[derive(Serialize, Deserialize)]
struct FilteredRecipesParams {
    diet_id: Option<i32>,
    cuisine_id: Option<i32>,
    meal_id: Option<i32>,
    ingredient_id: Option<i32>,
    limit: u64,
    author_id: Option<i32>,
    public: Option<bool>,
    collection_id: Option<i32>,
    page_offset: Option<u64>,
}

async fn get_filtered_recipes(Json(params): Json<FilteredRecipesParams>) -> (StatusCode, Json<Vec<recipe::Model>>) {
    println!("Start get filtered recipes");

    let db = db_conn().await.unwrap();
    let recipes = recipe::Entity::find()
        // Cuisine
        .apply_if(params.cuisine_id, |mut query, v| {
            query.filter(
                Condition::any().add(
                    recipe::Column::Id.in_subquery(
                        Query::select()
                            .column(recipe_cuisine::Column::RecipeId)
                            .and_where(recipe_cuisine::Column::CuisineId.eq(v))
                            .from(recipe_cuisine::Entity)
                            .to_owned(),
                    ),
                ),
            )
        })
        // Diet
        .apply_if(params.diet_id, |mut query, v| {
            query.filter(
                Condition::any().add(
                    recipe::Column::Id.in_subquery(
                        Query::select()
                            .column(recipe_diet::Column::RecipeId)
                            .and_where(recipe_diet::Column::DietId.eq(v))
                            .from(recipe_diet::Entity)
                            .to_owned(),
                    ),
                ),
            )
        })
        // Meal
        .apply_if(params.meal_id, |mut query, v| {
            query.filter(
                Condition::any().add(
                    recipe::Column::Id.in_subquery(
                        Query::select()
                            .column(recipe_meal::Column::RecipeId)
                            .and_where(recipe_meal::Column::MealId.eq(v))
                            .from(recipe_meal::Entity)
                            .to_owned(),
                    ),
                ),
            )
        })
        // Ingredient
        .apply_if(params.ingredient_id, |mut query, v| {
            query.filter(
                Condition::any().add(
                    recipe::Column::Id.in_subquery(
                        Query::select()
                            .column(recipe_ingredient::Column::RecipeId)
                            .and_where(recipe_ingredient::Column::IngredientId.eq(v))
                            .from(recipe_ingredient::Entity)
                            .to_owned(),
                    ),
                ),
            )
        })
        // Collection id
        /* .apply_if(params.collection_id, |mut query, v| {
            query.filter(
                Condition::any().add(
                    recipe::Column::Id.in_subquery(
                        Query::select()
                            .column(recipe_collection_recipe::Column::RecipeId)
                            .and_where(recipe_collection_recipe::Column::CollectionId.eq(v))
                            .from(recipe_collection_recipe::Entity)
                            .to_owned(),
                    ),
                ),
            )
        })
        // Author
        .apply_if(params.author_id, |mut query, v| {
            query.filter(
                recipe::Column::AuthorId.eq(v),
            )
        })
        // Public
        .apply_if(params.public, |mut query, v| {
            query.filter(
                recipe::Column::Public.eq(v),
            )
        }) */
        .limit(params.limit)
        /* .apply_if(Some(params.limit), QuerySelect::limit::<Option<u64>>) */
        .apply_if(Some(params.page_offset), QuerySelect::offset::<Option<u64>>)
        .all(&db)
        .await
        .unwrap();

        println!("Got filtered recipes");
    (StatusCode::OK, axum::Json(recipes))
}