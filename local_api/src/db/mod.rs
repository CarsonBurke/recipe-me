use sea_orm::{Database, DatabaseConnection, DbErr};

use crate::constants::DATABASE_URL;

pub async fn db_conn() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await;
    db
}