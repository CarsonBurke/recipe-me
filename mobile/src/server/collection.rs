use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect,
    RelationTrait,
};

use crate::{
    entities::{recipe, recipe_collection, recipe_collection_recipe},
    server::db_conn,
};

pub async fn test_find_collection(collection_id: i32) -> Option<recipe_collection::Model> {
    let db = db_conn().await.unwrap();
    let collection = recipe_collection::Entity::find_by_id(collection_id)
        .one(&db)
        .await
        .unwrap();

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

pub async fn new_collection(collection_name: String, description: String) -> Result<(), DbErr> {
    let db = db_conn().await.unwrap();

    let collection = recipe_collection::ActiveModel {
        collection_name: ActiveValue::Set(collection_name),
        description: ActiveValue::Set(Some(description)),
        ..Default::default()
    };

    let collection_result = collection.insert(&db).await?;
    println!("collection creation attempt: {:#?}", collection_result);

    Ok(())
}

pub async fn get_collection(id: i32) -> Option<recipe_collection::Model> {
    let db = db_conn().await.unwrap();
    let collections = recipe_collection::Entity::find_by_id(id)
        .one(&db)
        .await
        .unwrap();
    collections
}

pub async fn my_collection_recipes(collection_id: i32) -> Vec<recipe::Model> {
    let db = db_conn().await.unwrap();
    let recipes = recipe::Entity::find()
        .join(
            sea_orm::JoinType::InnerJoin,
            recipe::Relation::RecipeCollectionRecipe.def(),
        )
        .filter(recipe_collection_recipe::Column::CollectionId.eq(collection_id))
        .all(&db)
        .await
        .unwrap();

    recipes
}

pub async fn delete_collection(id: i32) -> Result<(), DbErr> {
    let db = db_conn().await.unwrap();
    recipe_collection::Entity::delete_by_id(id).exec(&db).await.unwrap();
    Ok(())
}