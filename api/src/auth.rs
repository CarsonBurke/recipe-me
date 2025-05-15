//! This crate contains all shared fullstack server functions.
use std::time::Duration;

use crate::entities::{
    comment, cuisine_name, diet_name, ingredient_name, login_token, meal_name, prelude::LoginToken,
    recipe_cuisine, recipe_diet, recipe_ingredient, recipe_meal, user,
};
use crate::{
    constants::LOGIN_TOKEN_MAX_AGE,
    data::{
        PartialCombinedRecipeIngredient, PartialComment, PartialCuisine, PartialDiet, PartialMeal,
    },
};
use dioxus::{html::g::offset, prelude::*};
use hmac::{Hmac, Mac};
use lettre::{
    Message, SmtpTransport, Transport,
    message::{MultiPart, SinglePart, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use scrypt::{
    Scrypt,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
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
use crate::db::db_conn;
use crate::secrets::EMAIL_SECRET;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerLoginToken {
    pub user_id: i32,
    pub token: String,
}

impl From<login_token::Model> for ServerLoginToken {
    fn from(value: login_token::Model) -> Self {
        ServerLoginToken {
            user_id: value.user_id,
            token: value.token,
        }
    }
}

impl ToString for ServerLoginToken {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[server]
pub async fn login(email: String, password: String) -> Result<ServerLoginToken, ServerFnError> {
    println!("Try to login");
    let db = db_conn().await.unwrap();
    let user: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Email.eq(email.clone()))
        .one(&db)
        .await?;

    let Some(user) = user else {
        return Err(ServerFnError::ServerError("User not found".to_string()));
    };
    println!("Found user with id {}", user.id);
    // Confirm that the passwords match

    let password_hash = PasswordHash::new(&user.password).unwrap();
    let Ok(_) = Scrypt.verify_password(password.as_bytes(), &password_hash) else {
        return Err(ServerFnError::ServerError("Incorrect password".to_string()));
    };

    // Create a login token

    Ok(create_or_update_login_token(user.id).await)
}

pub async fn create_account(
    username: String,
    email: String,
    password: String,
) -> Result<ServerLoginToken, ServerFnError> {
    println!("try to create account");
    let db = db_conn().await.unwrap();

    // Make sure no users already exist with this email

    let existing_user = user::Entity::find()
        .filter(user::Column::Email.eq(email.clone()))
        .one(&db)
        .await
        .unwrap();
    if existing_user.is_some() {
        return Err(ServerFnError::ServerError(
            "User with email already exists".to_string(),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    println!("going to hash password {password} with salt {salt}");
    let hashed_password = Scrypt
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    println!("Hashed password: {}", hashed_password);
    // Create the user

    let user = user::ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(username.clone()),
        email: ActiveValue::Set(email.clone()),
        password: ActiveValue::Set(hashed_password.clone()),
    };
    let result = user.insert(&db).await.unwrap();
    let user_id = result.id;
    println!("Created user with id {}", user_id);
    // Generate a token using the user id to allow for quick "login"

    let token = create_login_token(user_id).await;

    Ok(token)
}

/* type HmacSha256 = Hmac<Sha256>; */

pub async fn create_or_update_login_token(user_id: i32) -> ServerLoginToken {
    // Check if there is an existing token

    let db = db_conn().await.unwrap();
    let token = login_token::Entity::find_by_id(user_id)
        .one(&db)
        .await
        .unwrap();

    // Check if the token exists
    if let Some(token) = token {
        // The token exists
        // if it is valid, return it

        if is_token_valid(token.created_epoch) {
            return token.into();
        }

        // The current token is invalid, delete it so we can make a new one

        login_token::Entity::delete_by_id(user_id)
            .exec(&db)
            .await
            .unwrap();
    }

    // Create a token

    create_login_token(user_id).await
}

pub async fn create_login_token(user_id: i32) -> ServerLoginToken {
    let db = db_conn().await.unwrap();
    let existing_token = login_token::Entity::find_by_id(user_id)
        .one(&db)
        .await
        .unwrap();
    if let Some(existing_token) = existing_token {
        if is_token_valid(existing_token.created_epoch) {
            return existing_token.into();
        }
    }

    let random_bytes: [u8; 16] = rand::random();
    let token = hex::encode(random_bytes);

    let server_token = login_token::ActiveModel {
        token: ActiveValue::Set(token.clone()),
        user_id: ActiveValue::Set(user_id),
        created_epoch: ActiveValue::Set(Utc::now().timestamp() as i32),
    };
    let _ = server_token.insert(&db).await.unwrap();

    ServerLoginToken { user_id, token }
}

#[server]
pub async fn server_is_logged_in(token: ServerLoginToken) -> Result<bool, ServerFnError> {
    println!("server is logged in check");
    Ok(is_logged_in(token).await)
}

pub async fn is_logged_in(token: ServerLoginToken) -> bool {
    println!("is logged in check");

    let db = db_conn().await.unwrap();
    let token = login_token::Entity::find_by_id(token.user_id)
        .one(&db)
        .await
        .unwrap();

    println!("Tried to find existing token");

    // Check if the token exists
    let Some(token) = token else {
        println!("Token does not exist");
        return false;
    };

    is_token_valid(token.created_epoch)
}

pub fn is_token_valid(created_epoch: i32) -> bool {
    let now = Utc::now().timestamp();
    let diff = now - created_epoch as i64;

    // Check if the token is expired
    if diff > LOGIN_TOKEN_MAX_AGE {
        println!("Token is expired");
        return false;
    }
    println!("token is valid");
    true
}

pub fn signup_confirm_email(username: String, email: String, token: String) -> () {
    let html = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
            <head>
                <title>Welcome to Recipe Me {username}</title>
            </head>
            <body>
                <h1>You've created a new account with Recipe Me!</h1>
                <p>To continue the process, please click the link below. If you did not create an account with Recipe Me, please ignore this email.</p>
                <a href="/signup_confirm?token={token}" style="background-color: #4CAF50; border-radius: 6px; border: none; color: white; padding: 15px 32px; text-align: center; text-decoration: none; display: inline-block; font-size: 16px;">Verify Login</a>
                <p>Why are we asking you to verify your login? It allows us to ensure that your accountis safe and secure.</p>
            </body>
        </html>"#,
    );

    let email = Message::builder()
        .from("Recipe Me <carsonburke22@gmail.com>".parse().unwrap())
        // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to(format!("Hei <{}>", email).parse().unwrap())
        .subject(format!("Welcome to Recipe Me {username} 👋"))
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_PLAIN)
                        .body(format!("Welcome to Recipe Me {username}!")),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(html.to_string()),
                ),
        )
        .unwrap();

    let creds = Credentials::new("carsonburke22".to_owned(), EMAIL_SECRET.to_owned());

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
    use super::is_logged_in;

    #[test]
    fn test_is_logged_in() {}
}
