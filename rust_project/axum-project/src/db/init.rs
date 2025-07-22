use std::env;

use sea_orm::{Database, DatabaseConnection};

pub async fn init_db() -> DatabaseConnection {
    match Database::connect(env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file"))
        .await
    {
        Ok(db) => db,
        Err(e) => panic!("Failed to connect to database: {e}"),
    }
}
