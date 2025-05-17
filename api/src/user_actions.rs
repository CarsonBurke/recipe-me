use std::str::FromStr;

use dioxus::prelude::{
    server_fn::error::{NoCustomError, WrapError},
    *,
};
use sea_orm::{prelude::Decimal, ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{
    CustomServerError,
    auth::{ServerLoginToken, is_logged_in},
    db::db_conn,
    entities::{ingredient_name, recipe, recipe_collection, recipe_ingredient},
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NewIngredient {
    pub name: String,
    pub amount: Option<f32>,
    pub description: String,
}

#[server]
pub async fn create_recipe(
    login_token: ServerLoginToken,
    name: String,
    description: String,
    instructions: String,
    ingredients: Vec<NewIngredient>,
) -> Result<i32, ServerFnError> {
    let db = db_conn().await.unwrap();

    if !is_logged_in(&login_token).await {
        return Err(ServerFnError::WrappedServerError(NoCustomError));
    }

    let recipe = recipe::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(name),
        description: ActiveValue::Set(description.clone()),
        summary: ActiveValue::Set({
            let mut cloned_description = description.clone();
            cloned_description.shrink_to(100);
            cloned_description
        }),
        instructions: ActiveValue::Set(instructions),
        author_id: ActiveValue::Set(Some(login_token.user_id)),
        views: ActiveValue::Set(Some(0)),
        ratings: ActiveValue::Set(0),
        total_rating: ActiveValue::Set(0),
        source: ActiveValue::NotSet,
        public: ActiveValue::Set(Some(false)),
    };
    let recipe_result = recipe.insert(&db).await.unwrap();

    for ingredient in ingredients {
        let ingredient_name = ingredient_name::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(ingredient.name),
            affiliate_link: ActiveValue::NotSet,
        };
        let ingredient_name_result = ingredient_name.insert(&db).await.unwrap();

        let recipe_ingredient = recipe_ingredient::ActiveModel {
            ingredient_id: ActiveValue::Set(ingredient_name_result.id),
            description: ActiveValue::Set(ingredient.description),
            amount: ActiveValue::Set(
                Decimal::from_f32_retain(ingredient.amount.unwrap_or(1.0)).expect("Invalid amount"),
            ),
            recipe_id: ActiveValue::Set(recipe_result.id),
        };

        let _ = recipe_ingredient.insert(&db).await.unwrap();
    }

    Ok(recipe_result.id)
}

#[server]
pub async fn get_collections(
    login_token: ServerLoginToken,
) -> Result<Vec<recipe_collection::Model>, ServerFnError> {
    if !is_logged_in(&login_token).await {
        return Err(ServerFnError::WrappedServerError(NoCustomError));
    }

    let db = db_conn().await.unwrap();
    let collections = recipe_collection::Entity::find()
        .filter(recipe_collection::Column::UserId.eq(login_token.user_id))
        .all(&db)
        .await
        .unwrap();
    Ok(collections)
}
