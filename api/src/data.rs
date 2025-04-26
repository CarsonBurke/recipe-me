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