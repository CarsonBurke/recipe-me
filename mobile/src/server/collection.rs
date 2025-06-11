use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

use crate::{entities::recipe_collection, server::db_conn};

pub async fn test_find_collection(collection_id: i32) -> Option<recipe_collection::Model> {
    let db = db_conn().await.unwrap();
    let collection = recipe_collection::Entity::find_by_id(collection_id).one(&db).await.unwrap();

    println!("collection: {:#?}", collection);
    collection
}

pub async fn create_collection(collection_name: String) {
    let db = db_conn().await.unwrap();
    let collection = recipe_collection::ActiveModel {
        collection_name: ActiveValue::Set(collection_name),

        ..Default::default()
    };

    let collection_result = collection.insert(&db).await.unwrap();
    println!("collection creation attempt: {:#?}", collection_result);
}

pub async fn get_my_collections() -> Vec<recipe_collection::Model> {
    let db = db_conn().await.unwrap();

    let collections = recipe_collection::Entity::find().all(&db).await.unwrap();
    println!("collections: {:#?}", collections);
    collections
}