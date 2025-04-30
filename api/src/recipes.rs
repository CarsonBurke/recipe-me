use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement};

use crate::{constants::DB_NAME, secrets::DATABASE_URL};

pub async fn init_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let conn = match db.get_database_backend() {
        DbBackend::Postgres => {
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
        }
        _ => {
            panic!("Unsupported database backend");
        }
    };

    Ok(conn)
}