pub mod sample_data;

use sea_orm::{ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbBackend, DbErr, Statement};

use crate::secrets::DATABASE_URL;

pub async fn db_conn() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(format!("../{DATABASE_URL}")).await;
    db
    /* let conn = match db.get_database_backend() {
        /* DbBackend::Postgres => {
            /* db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", DB_NAME),
            ))
            .await?; */
        
             let url = format!("{}/{}", DATABASE_URL, DB_NAME);
             Database::connect(&url).await?
        } */
        DatabaseBackend::Sqlite => {
            Database::connect(&url).await?;
        }
        _ => {
            panic!("Unsupported database backend");
        }
    };

    Ok(conn) */
}