use api::data::PartialCombinedRecipeIngredient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct IngredientPartial {
    pub name: String,
    pub amount: f32,
    pub description: String,
}

impl From<&PartialCombinedRecipeIngredient> for IngredientPartial {
    fn from(value: &PartialCombinedRecipeIngredient) -> Self {
        Self {
            name: value.name.clone(),
            amount: value.amount,
            description: value.description.clone(),
        }
    }
}