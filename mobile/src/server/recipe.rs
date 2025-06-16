use sea_orm::{metric::Info, prelude::Decimal, sea_query::Query, ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter, QuerySelect, QueryTrait};
use serde::{Deserialize, Serialize};

use crate::{
    data::partials::IngredientPartial,
    entities::{ingredient_name, recipe, recipe_cuisine, recipe_diet, recipe_ingredient, recipe_meal},
    server::db_conn,
};

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

pub async fn get_filtered_recipes(
    params: FilteredRecipesParams,
) -> Vec<recipe::Model> {
    println!("Start get filtered recipes");

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
    recipes
}

/// Returns the recipe of the id
pub async fn create_recipe(
    name: String,
    description: String,
    instructions: String,
    ingredients: Vec<IngredientPartial>,
) -> i32 {

    let active_recipe = recipe::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(name),
        description: ActiveValue::Set(description.clone()),
        summary: ActiveValue::Set({
            let mut cloned_description = description.clone();
            cloned_description.shrink_to(100);
            cloned_description
        }),
        instructions: ActiveValue::Set(instructions),
        author_id: ActiveValue::Set(None),
        views: ActiveValue::Set(Some(0)),
        ratings: ActiveValue::Set(0),
        total_rating: ActiveValue::Set(0),
        source: ActiveValue::NotSet,
        public: ActiveValue::Set(Some(false)),
        ..Default::default()
    };

    let recipe_result = active_recipe.insert(&db_conn().await.unwrap()).await;
    let recipe_id = recipe_result.unwrap().id;

    create_recipe_ingredients(recipe_id, &ingredients).await;

    recipe_id
}

pub async fn create_recipe_ingredients(
    recipe_id: i32,
    ingredients: &[IngredientPartial],
) -> Vec<i32> {
    let mut ingredient_ids = Vec::new();

    for ingredient in ingredients {
        let active_ingredient_name = ingredient_name::ActiveModel {
            name: ActiveValue::Set(ingredient.name.clone()),
            ..Default::default()
        };

        let name_result = active_ingredient_name
            .insert(&db_conn().await.unwrap())
            .await;

        let active_ingredient = recipe_ingredient::ActiveModel {
            recipe_id: ActiveValue::Set(recipe_id),
            amount: ActiveValue::Set(Decimal::from_f32_retain(ingredient.amount).unwrap()),
            description: ActiveValue::Set(ingredient.description.clone()),
            ..Default::default()
        };

        let ingredient_result = active_ingredient.insert(&db_conn().await.unwrap()).await;

        ingredient_ids.push(name_result.unwrap().id);
    }

    ingredient_ids
}
