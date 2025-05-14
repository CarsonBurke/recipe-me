//! This crate contains all shared fullstack server functions.
use std::time::Duration;

use data::{
    PartialCombinedRecipeIngredient, PartialComment, PartialCuisine, PartialDiet, PartialMeal,
};
use dioxus::{html::g::offset, prelude::*};
use entities::{
    comment, cuisine_name, diet_name, ingredient_name, login_token, meal_name, prelude::LoginToken,
    recipe_cuisine, recipe_diet, recipe_ingredient, recipe_meal, user,
};
use hmac::{Hmac, Mac};
use lettre::{
    Message, SmtpTransport, Transport,
    message::{MultiPart, SinglePart, header::ContentType},
    transport::smtp::authentication::Credentials,
};
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
use serde::{Deserialize, Serialize};
use sha2::Sha256;

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
    pub cuisine_id: Option<i32>,
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

#[server]
pub async fn login(username_or_email: String, password: String) -> Result<String, ServerFnError> {
    let db = db_conn().await.unwrap();
    let user: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Username.eq(username_or_email.clone()))
        .filter(user::Column::Email.eq(username_or_email.clone()))
        .one(&db)
        .await?;

    let Some(user) = user else {
        return Err(ServerFnError::ServerError("User not found".to_string()));
    };

    let id = user.id;

    Ok(create_login_token(user.id).await)
}

#[server]
pub async fn create_account(
    username: String,
    email: String,
    password: String,
) -> Result<String, ServerFnError> {
    let db = db_conn().await.unwrap();

    // Make sure no users already exist with this email

    let existing_user = user::Entity::find()
        .filter(user::Column::Email.eq(email.clone()))
        .one(&db)
        .await
        .unwrap();
    if existing_user.is_some() {
        return Err(ServerFnError::ServerError(
            "User already exists".to_string(),
        ));
    }

    // Create the user

    let user = user::ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(username.clone()),
        email: ActiveValue::Set(email.clone()),
        password: ActiveValue::Set(password.clone()),
    };
    let result = user.insert(&db).await.unwrap();
    let user_id = result.id;

    // Generate a token using the user id to allow for quick "login"

    let token = create_login_token(user_id).await;
    Ok(token)
}

/* type HmacSha256 = Hmac<Sha256>; */

pub async fn create_login_token(user_id: i32) -> String {
    /* dotenv::dotenv().ok();

    let secret = std::env::var("LOGIN_TOKEN_SECRET").unwrap();

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(user_id.to_string().as_bytes());

    let result = mac.finalize();
    let code = result.into_bytes();

    "".to_string() */

    let random_bytes: [u8; 16] = rand::random();
    let token = hex::encode(random_bytes);

    let db = db_conn().await.unwrap();

    let server_token = login_token::ActiveModel {
        token: ActiveValue::Set(token.clone()),
        user_id: ActiveValue::Set(user_id),
        created_epoch: ActiveValue::Set(Utc::now().timestamp() as i32),
    };
    let result = server_token.insert(&db).await.unwrap();

    token
}

pub fn signup_confirm_email(username: String, email: String) -> () {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Welcome to Recipe Me</title>
    </head>
    <body>
        <h1>You've created a new account with Recipe Me!</h1>
        <p>To continue the process, please click the link below. If you did not create an account with Recipe Me, please ignore this email.</p>
        <a href="{}" style="background-color: #4CAF50; border-radius: 6px; border: none; color: white; padding: 15px 32px; text-align: center; text-decoration: none; display: inline-block; font-size: 16px;">Verify Login</a>
        <p>Why are we asking you to verify your login? It allows us to ensure that your accountis safe and secure.</p>
    </body>
</html>"#,
        email
    );

    let email = Message::builder()
        .from("Recipe Me <carsonburke22@gmail.com>".parse().unwrap())
        // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to(format!("Hei <{}>", email).parse().unwrap())
        .subject(format!("Welcome to Recipe Me {username}"))
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_PLAIN)
                        .body("Welcome to Recipe Me!".to_string()),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(html.to_string()),
                ),
        )
        .unwrap();

    let creds = Credentials::new("carsonburke22".to_owned(), "oxsw dypy gkoh kwze".to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let res = mailer.send(&email);
    println!("res {res:?}");
}

#[cfg(test)]
mod test {
    use crate::{create_login_token, signup_confirm_email};

    #[test]
    fn test_signup_confirm_email() {
        signup_confirm_email(
            "marvin22".to_string(),
            "carsonburke22@gmail.com".to_string(),
        );
    }

    #[tokio::test]
    async fn test_create_login_token() {
        create_login_token(rand::random_range(0..50000)).await;
    }
}
