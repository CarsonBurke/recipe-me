//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, EntityTrait};
// use sqlx::{postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgRow}, Connection, PgConnection};

use recipes::init_db;

use self::entities::recipe;

mod data;
mod sample;
mod constants;
mod recipes;
mod entities;
mod secrets;

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
    let db = init_db().await.unwrap();
    let recipes = recipe::Entity::find().all(&db).await.unwrap();
    Ok(recipes)

    // Ok(vec![])
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