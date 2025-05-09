//! This crate contains all shared fullstack server functions.
use data::{PartialCombinedRecipeIngredient, PartialComment, PartialCousine, PartialDiet, PartialMeal};
use dioxus::{html::g::offset, prelude::*};
use entities::{
    comment, cousine_name, diet_name, ingredient_name, meal_name, recipe_cousine, recipe_diet, recipe_ingredient, recipe_meal, user
};
use sea_orm::{
    ColumnTrait, Condition, ConnectOptions, Database, DatabaseConnection, EntityTrait,
    FromQueryResult, JoinType, QueryFilter, QuerySelect, QueryTrait, RelationTrait,
    prelude::Expr,
    sea_query::{IntoCondition, Query},
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilteredRecipesParams {
    pub ingredient_id: Option<i32>,
    pub cousine_id: Option<i32>,
    pub diet_id: Option<i32>,
    pub meal_id: Option<i32>,
    pub limit: u64,
    pub page_offset: Option<u64>,
}

#[server]
pub async fn get_filtered_recipes(
    params: FilteredRecipesParams,
) -> Result<Vec<recipe::Model>, ServerFnError> {
    println!("Server side params {:#?}", params);

    let db = db_conn().await.unwrap();
    let recipes = recipe::Entity::find()
        // Cousine
        .apply_if(params.cousine_id, |mut query, v| {
            query.filter(
                Condition::any().add(
                    recipe::Column::Id.in_subquery(
                        Query::select()
                            .column(recipe_cousine::Column::RecipeId)
                            .and_where(recipe_cousine::Column::CousineId.eq(v))
                            .from(recipe_cousine::Entity)
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
        .limit(params.limit)
        /* .apply_if(Some(params.limit), QuerySelect::limit::<Option<u64>>) */
        .apply_if(Some(params.page_offset), QuerySelect::offset::<Option<u64>>)
        .all(&db)
        .await
        .unwrap();

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
pub async fn get_recipe_cousines(id: i32) -> Result<Vec<PartialCousine>, ServerFnError> {
    let db = db_conn().await.unwrap();
    let recipe_cousines = recipe_cousine::Entity::find()
        .join(
            JoinType::InnerJoin,
            recipe_cousine::Relation::CousineName.def(),
        )
        .filter(recipe_cousine::Column::RecipeId.eq(id))
        .select_only()
        .column_as(cousine_name::Column::Name, "name")
        .column_as(recipe_cousine::Column::CousineId, "id")
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
