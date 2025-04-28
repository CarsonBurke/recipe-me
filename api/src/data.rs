use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// pub struct Recipe {
//     pub id: i32,
//     pub name: String,
//     pub description: String,
//     pub instructions: String,
//     pub ingredients: Vec<String>,
//     pub views: i32,
//     pub ratings: i32,
//     pub total_rating: i32,
//     pub cousine_id: CousineId,
//     pub recipe_types: Vec<RecipeType>
// }

#[derive(Serialize, Deserialize)]
pub enum CousineId {
    Italian,
    Mexican,
    Chinese
}

impl ToString for CousineId {
    fn to_string(&self) -> String {
        match self {
            CousineId::Italian => "Italian".to_string(),
            CousineId::Mexican => "Mexican".to_string(),
            CousineId::Chinese => "Chinese".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum RecipeType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
    Sauce,
    Side,
    Dessert
}

impl ToString for RecipeType {
    fn to_string(&self) -> String {
        match self {
            RecipeType::Breakfast => "Breakfast".to_string(),
            RecipeType::Lunch => "Lunch".to_string(),
            RecipeType::Dinner => "Dinner".to_string(),
            RecipeType::Snack => "Snack".to_string(),
            RecipeType::Sauce => "Sauce".to_string(),
            RecipeType::Side => "Side".to_string(),
            RecipeType::Dessert => "Dessert".to_string(),
        }
    }
}