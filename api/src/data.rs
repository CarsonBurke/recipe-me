use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialCombinedRecipeIngredient {
    pub name: String,
    pub amount: i32,
    pub description: String
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialMeal {
    pub name: String,
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialCousine {
    pub name: String,
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct PartialDiet {
    pub name: String,
}