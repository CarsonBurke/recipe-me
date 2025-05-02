use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialCombinedRecipeIngredient {
    pub name: String,
    pub amount: i32,
    pub description: String,
    pub id: i32,
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialMeal {
    pub name: String,
    pub id: i32,
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialCousine {
    pub name: String,
    pub id: i32,
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialDiet {
    pub name: String,
    pub id: i32,
}