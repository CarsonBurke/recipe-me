//! This crate contains all shared fullstack server functions.
use data::{PartialCombinedRecipeIngredient, PartialCousine, PartialDiet, PartialMeal};
use dioxus::prelude::*;
use entities::{cousine_name, diet_name, ingredient_name, meal_name, recipe_cousine, recipe_diet, recipe_ingredient, recipe_meal};
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QueryFilter,
    ColumnTrait, RelationTrait, QuerySelect, 
};
// use sqlx::{postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgRow}, Connection, PgConnection};
use db::db_conn;
use serde::{Deserialize, Serialize};

use self::entities::recipe;

pub mod constants;
pub mod data;
pub mod db;
pub mod entities;
mod sample;
pub mod secrets;

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

// pub async fn db() -> DatabaseConnection {
//     let opt = ConnectOptions::new("protocol://username:password@host/database");
//     let db = Database::connect(opt).await.unwrap();
//     db
// }

#[server(Recipes)]
pub async fn get_recipes() -> Result<Vec<recipe::Model>, ServerFnError> {
    let db = db_conn().await.unwrap();
    let recipes = recipe::Entity::find().all(&db).await.unwrap();
    Ok(recipes)

    // Ok(vec![])
}

#[server]
pub async fn get_recipe(id: i32) -> Result<recipe::Model, ServerFnError> {
    let db = db_conn().await.unwrap();
    let recipe = recipe::Entity::find_by_id(id).one(&db).await.unwrap();
    Ok(recipe.unwrap())
}

#[server]
pub async fn get_recipe_ingredients(
    id: i32,
) -> Result<Vec<PartialCombinedRecipeIngredient>, ServerFnError> {
    let db = db_conn().await.unwrap();
    // let recipe_ingredients = recipe_ingredient::Entity::find().filter(recipe_ingredient::Column::RecipeId.eq(id)).all(&db).await.unwrap();

    let partial = recipe_ingredient::Entity::find()
        .join(
            JoinType::InnerJoin,
            recipe_ingredient::Relation::IngredientName.def(),
        )
        .filter(recipe_ingredient::Column::RecipeId.eq(id))
        .column_as(ingredient_name::Column::Name, "name")
        .column_as(recipe_ingredient::Column::Amount, "amount")
        .column_as(recipe_ingredient::Column::Description, "description")
        .into_model::<PartialCombinedRecipeIngredient>()
        .all(&db).await.unwrap();

    Ok(partial)
}

#[server]
pub async fn get_recipe_cousines(id: i32) -> Result<Vec<PartialCousine>, ServerFnError> {
    let db = db_conn().await.unwrap();
    let recipe_cousines = recipe_cousine::Entity::find()
        .join(
            JoinType::InnerJoin,
            recipe_cousine::Relation::CousineName.def(),
        )
        .filter(recipe_cousine::Column::RecipeId.eq(id))
        .column_as(cousine_name::Column::Name, "name")
        .into_model::<PartialCousine>()
        .all(&db)
        .await
        .unwrap();
    Ok(recipe_cousines)
}

#[server]
pub async fn get_recipe_meals(id: i32) -> Result<Vec<PartialMeal>, ServerFnError> {
    let db = db_conn().await.unwrap();
    let recipe_meals = recipe_meal::Entity::find()
        .join(
            JoinType::InnerJoin,
            recipe_meal::Relation::MealName.def(),
        )
        .filter(recipe_meal::Column::RecipeId.eq(id))
        .column_as(meal_name::Column::Name, "name")
        .into_model::<PartialMeal>()
        .all(&db)
        .await
        .unwrap();
    Ok(recipe_meals)
}

#[server]
pub async fn get_recipe_diets(id: i32) -> Result<Vec<PartialDiet>, ServerFnError> {
    let db = db_conn().await.unwrap();
    let recipe_diets = recipe_diet::Entity::find()
        .join(
            JoinType::InnerJoin,
            recipe_diet::Relation::DietName.def(),
        )
        .filter(recipe_diet::Column::RecipeId.eq(id))
        .column_as(diet_name::Column::Name, "name")
        .into_model::<PartialDiet>()
        .all(&db)
        .await
        .unwrap();
    Ok(recipe_diets)
}  

//#[server(Recipes)]
// #[server]
// pub async fn get_recipes() -> Result<Vec<Recipe>, ServerFnError> {
//     let mut x = PgConnection::connect(&format!("postgres://postgres:{DB_PASSWORD}@localhost/db.db")).await.unwrap();

//     let recipes = sqlx::query_as!(Recipe, "SELECT * FROM recipes").fetch_all(&mut x).await.map_err(|_| ServerFnError::ServerError("Error fetching recipes".to_string()));

//     recipes
// }

// #[cfg(features = "server")]
// thread_local! {
//     pub static DB: PgPool = {
//         let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect("postgres://postgres:postgres@localhost/recipes").await.unwrap();

//         pool
//     }
// }

// #[cfg(features = "server")]
// thread_local! {
//     pub static DB {
//         let mut opt = ConnectOptions::new("postgres://postgres:postgres@localhost/recipes");
//         opt.max_connections(100)
//             .min_connections(5)
//             .connect_timeout(Duration::from_secs(8))
//             .acquire_timeout(Duration::from_secs(8))
//             .idle_timeout(Duration::from_secs(8))
//             .max_lifetime(Duration::from_secs(8))
//             .sqlx_logging(true)
//             .sqlx_logging_level(log::LevelFilter::Info)
//             .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

//         let db = Database::connect(opt).await?;
//         db
//     }
// }
