use api::data::{PartialCombinedRecipeIngredient, PartialComment, PartialCuisine, PartialDiet, PartialMeal};
use sea_orm::{metric::Info, prelude::Decimal, sea_query::Query, ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QuerySelect, QueryTrait, RelationTrait};
use serde::{Deserialize, Serialize};

use crate::{
    data::partials::IngredientPartial,
    entities::{comment, cuisine_name, diet_name, ingredient_name, meal_name, recipe, recipe_cuisine, recipe_diet, recipe_ingredient, recipe_meal, user},
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
) -> Result<i32, DbErr> {
    let db = db_conn().await.unwrap();

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

    let new_recipe = active_recipe.insert(&db).await?;

    create_recipe_ingredients(new_recipe.id, &ingredients, &db).await;

    Ok(new_recipe.id)
}

pub async fn create_recipe_ingredients(
    recipe_id: i32,
    ingredients: &[IngredientPartial],
    db: &DatabaseConnection
) {
    for ingredient in ingredients {
        let active_ingredient_name = ingredient_name::ActiveModel {
            name: ActiveValue::Set(ingredient.name.clone()),
            ..Default::default()
        };

        let name_result = active_ingredient_name
            .insert(db)
            .await;

        let active_ingredient = recipe_ingredient::ActiveModel {
            recipe_id: ActiveValue::Set(recipe_id),
            amount: ActiveValue::Set(Decimal::from_f32_retain(ingredient.amount).unwrap()),
            description: ActiveValue::Set(ingredient.description.clone()),
            ..Default::default()
        };

        let ingredient_result = active_ingredient.insert(db).await;
    };
}

pub async fn recipes_count() -> u64 {
    let db = db_conn().await.unwrap();
    recipe::Entity::find().count(&db).await.unwrap()
}

pub async fn recipe_from_public(id: i32) -> Result<i32, DbErr> {
    
    // Get the recipe and relevant data from the public database
    // Then copy it over into ours
    /* let public_recipe = recipe::Entity::find_by_id(id)
        .one(&db)
        .await
        .unwrap(); */

    let public_recipe = api::get_recipe(id).await.unwrap();

    let public_ingredients = api::get_recipe_ingredients(id).await.unwrap();
    let ingredients = public_ingredients.iter().map(|ing| IngredientPartial::from(ing)).collect();

    let new_recipe_id = create_recipe(public_recipe.name, public_recipe.description, public_recipe.instructions, ingredients).await;
    new_recipe_id
}


pub async fn get_recipe(id: i32) -> Result<recipe::Model, DbErr> {
    let db = db_conn().await.unwrap();
    let recipe = recipe::Entity::find_by_id(id).one(&db).await.unwrap();
    Ok(recipe.unwrap())
}

pub async fn get_recipe_ingredients(
    id: i32,
) -> Result<Vec<PartialCombinedRecipeIngredient>, DbErr> {
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

pub async fn get_recipe_cuisines(id: i32) -> Result<Vec<PartialCuisine>, DbErr> {
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

pub async fn get_recipe_meals(id: i32) -> Result<Vec<PartialMeal>, DbErr> {
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

pub async fn get_recipe_diets(id: i32) -> Result<Vec<PartialDiet>, DbErr> {
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

pub async fn get_recipe_comments(recipe_id: i32, limit: u64) -> Result<Vec<PartialComment>, DbErr> {
    let db = db_conn().await.unwrap();
    let recipe_comments = comment::Entity::find()
        .limit(limit)
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

pub async fn delete_recipe(id: i32) -> Result<(), DbErr> {
    let db = db_conn().await.unwrap();
    recipe::Entity::delete_by_id(id).exec(&db).await.unwrap();
    Ok(())
}