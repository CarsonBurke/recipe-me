use sea_orm::{ConnectionTrait, DbBackend, DbErr, Schema};

use crate::entities;
use super::db_conn;

pub async fn create_tables() -> Result<(), DbErr> {
    let db = db_conn().await?;

    Ok(())
}

pub async fn create_sample_data() -> Result<(), DbErr> {
    let db = db_conn().await?;
    let db_backend = db.get_database_backend();

    let sqlite = DbBackend::Sqlite;
    let schema = Schema::new(sqlite);

    db.execute(db_backend.build(&schema.create_table_from_entity(entities::recipe::Entity))).await?;

    Ok(())
}