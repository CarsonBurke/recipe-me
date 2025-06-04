//! This crate contains all shared fullstack server functions.
use std::{fmt::Display, time::Duration};

use auth::ServerLoginToken;
use data::{
    PartialCollection, PartialCombinedRecipeIngredient, PartialComment, PartialCuisine, PartialDiet, PartialMeal
};
use dioxus::{html::g::offset, logger::tracing::info, prelude::{server_fn::error::NoCustomError, *}};
use entities::{
    comment, cuisine_name, diet_name, ingredient_name, login_token, meal_name, prelude::LoginToken, recipe_collection, recipe_collection_recipe, recipe_cuisine, recipe_diet, recipe_ingredient, recipe_meal, user
};
use hmac::{Hmac, Mac};
use lettre::{
    Message, SmtpTransport, Transport,
    message::{MultiPart, SinglePart, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use scrypt::{password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Scrypt};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, ConnectOptions, Database,
    DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QueryFilter, QuerySelect,
    QueryTrait, RelationTrait,
    prelude::Expr,
    sea_query::{IntoCondition, Query},
    sqlx::types::chrono::Utc,
};
// use sqlx::{postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgRow}, Connection, PgConnection};
use db::db_conn;
use secrets::EMAIL_SECRET;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use self::entities::recipe;

pub mod constants;
pub mod data;
pub mod db;
pub mod entities;
mod sample;
pub mod secrets;
pub mod auth;
pub mod user_actions;

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

#[derive(Debug, Serialize, Deserialize)]
pub enum CustomServerError {
    NotAuthorized,
}

impl std::error::Error for CustomServerError {
    
}

impl Display for CustomServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomServerError::NotAuthorized => write!(f, "Not Authorized"),
            _ => write!(f, "Custom server error - unknown"),
        }
    }
}

#[server(Recipes)]
pub async fn get_recipes() -> Result<Vec<recipe::Model>, ServerFnError> {
    let db = db_conn().await.unwrap();
    let recipes = recipe::Entity::find().all(&db).await.unwrap();
    Ok(recipes)
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilteredRecipesParams {
    pub ingredient_id: Option<i32>,
    pub cuisine_id: Option<i32>,
    pub diet_id: Option<i32>,
    pub meal_id: Option<i32>,
    pub limit: u64,
    pub page_offset: Option<u64>,
    pub author_id: Option<i32>,
    pub public: Option<bool>,
    pub collection_id: Option<i32>,
}

#[server]
pub async fn get_filtered_recipes(
    params: FilteredRecipesParams,
) -> Result<Vec<recipe::Model>, ServerFnError> {
    println!("Start get filtered recipes");
    info!("Start get filtered recipes");

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

        println!("Got filtered recipes: {}", recipes.len());
    Ok(recipes)
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
        // Instead can probably do .find_with_related(IngredientName)
        .join(
            JoinType::InnerJoin,
            recipe_ingredient::Relation::IngredientName.def(),
        )
        .filter(recipe_ingredient::Column::RecipeId.eq(id))
        .select_only()
        .column_as(ingredient_name::Column::Name, "name")
        .column_as(recipe_ingredient::Column::Amount, "amount")
        .column_as(recipe_ingredient::Column::Description, "description")
        .column_as(recipe_ingredient::Column::IngredientId, "id")
        .into_model::<PartialCombinedRecipeIngredient>()
        .all(&db)
        .await
        .unwrap();

    Ok(partial)
}

#[server]
pub async fn get_recipe_cuisines(id: i32) -> Result<Vec<PartialCuisine>, ServerFnError> {
    let db = db_conn().await.unwrap();
    let recipe_cuisines = recipe_cuisine::Entity::find()
        .join(
            JoinType::InnerJoin,
            recipe_cuisine::Relation::CuisineName.def(),
        )
        .filter(recipe_cuisine::Column::RecipeId.eq(id))
        .select_only()
        .column_as(cuisine_name::Column::Name, "name")
        .column_as(recipe_cuisine::Column::CuisineId, "id")
        .into_model::<PartialCuisine>()
        .all(&db)
        .await
        .unwrap();
    Ok(recipe_cuisines)
}

#[server]
pub async fn get_recipe_meals(id: i32) -> Result<Vec<PartialMeal>, ServerFnError> {
    let db = db_conn().await.unwrap();
    let recipe_meals = recipe_meal::Entity::find()
        .join(JoinType::InnerJoin, recipe_meal::Relation::MealName.def())
        .filter(recipe_meal::Column::RecipeId.eq(id))
        .select_only()
        .column_as(meal_name::Column::Name, "name")
        .column_as(recipe_meal::Column::MealId, "id")
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
        .join(JoinType::InnerJoin, recipe_diet::Relation::DietName.def())
        .filter(recipe_diet::Column::RecipeId.eq(id))
        .select_only()
        .column_as(diet_name::Column::Name, "name")
        .column_as(recipe_diet::Column::DietId, "id")
        .into_model::<PartialDiet>()
        .all(&db)
        .await
        .unwrap();
    Ok(recipe_diets)
}

#[server]
pub async fn get_recipe_comments(recipe_id: i32) -> Result<Vec<PartialComment>, ServerFnError> {
    let db = db_conn().await.unwrap();
    let recipe_comments = comment::Entity::find()
        .join(JoinType::InnerJoin, comment::Relation::Recipe.def())
        .filter(comment::Column::RecipeId.eq(recipe_id))
        .join(JoinType::InnerJoin, comment::Relation::User.def())
        .select_only()
        .column_as(user::Column::Username, "name")
        .column_as(comment::Column::UserId, "user_id")
        .column_as(comment::Column::Comment, "comment")
        .column_as(comment::Column::Rating, "rating")
        .into_model::<PartialComment>()
        .all(&db)
        .await
        .unwrap();
    Ok(recipe_comments)
}

#[server]
pub async fn get_collection(collection_id: i32) -> Result<recipe_collection::Model, ServerFnError> {
    let db = db_conn().await.unwrap();
    let collection = recipe_collection::Entity::find_by_id(collection_id).one(&db).await.unwrap();
    Ok(collection.unwrap())
}

/* #[server]
pub async fn create_user(
    username: String,
    email: String,
    password: String,
) -> Result<(), ServerFnError> {
    let db = db_conn().await.unwrap();
    let user = User::new(username, email, password);
    user.create(&db).await.unwrap();
    Ok(())
}

#[server]
pub async fn delete_user(
    username_or_email: String,
    password: String,
) -> Result<String, ServerFnError> {
    let db = db_conn().await.unwrap();
    let user = User::delete(&db, username_or_email, password)
        .await
        .unwrap();
    Ok(user.username)
} */

#[cfg(test)]
mod test {
    use crate::{auth::create_login_token, auth::signup_confirm_email};

    #[test]
    fn test_signup_confirm_email() {
        signup_confirm_email(
            "marvin22".to_string(),
            "carsonburke22@gmail.com".to_string(),
            "token".to_string()
        );
    }

    #[tokio::test]
    async fn test_create_login_token() {
        create_login_token(rand::random_range(0..50000)).await;
    }
}
