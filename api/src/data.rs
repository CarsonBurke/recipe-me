use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialCombinedRecipeIngredient {
    pub name: String,
    pub amount: f32,
    pub description: String,
    pub id: i32,
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialMeal {
    pub name: String,
    pub id: i32,
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialCuisine {
    pub name: String,
    pub id: i32,
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialDiet {
    pub name: String,
    pub id: i32,
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialComment {
    pub name: String,
    pub user_id: i32,
    pub comment: String,
    pub rating: i32,
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialCollection {
    pub name: String,
    pub id: i32,
}