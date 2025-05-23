//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "DietName")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::recipe_diet::Entity")]
    RecipeDiet,
}

impl Related<super::recipe_diet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeDiet.def()
    }
}

impl Related<super::recipe::Entity> for Entity {
    fn to() -> RelationDef {
        super::recipe_diet::Relation::Recipe.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::recipe_diet::Relation::DietName.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
