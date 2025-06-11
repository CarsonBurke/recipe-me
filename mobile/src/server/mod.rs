use std::time::{self, SystemTime, UNIX_EPOCH};

use dioxus::prelude::*;
use sea_orm::{Database, DatabaseConnection, DbErr, EntityTrait};

use crate::entities::recipe_collection;

pub const DATABASE_URL: &str = "sqlite://recipes.sqlite?mode=rwc";

pub async fn db_conn() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await;
    db
}

pub async fn test_db() {
    let db = db_conn().await.unwrap();

    
}

pub async fn test_find_collection(collection_id: i32) -> Option<recipe_collection::Model> {
    let db = db_conn().await.unwrap();
    let collection = recipe_collection::Entity::find_by_id(collection_id).one(&db).await.unwrap();

    println!("collection: {:#?}", collection);
    collection
}

#[server]
pub async fn ping_self() -> Result<u128, ServerFnError> {
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("ping self since epoch {}", since_epoch.as_millis());
    Ok(since_epoch.as_millis())
}

#[server]
pub async fn ping_net_server() -> Result<u128, ServerFnError> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    // let req = requests::get("http://localhost:3000/ping").await.unwrap();

    println!("ping net server since epoch {}", current_time.as_millis());
    Ok(current_time.as_millis())
    /* let diff;

    Ok(diff) */
}
