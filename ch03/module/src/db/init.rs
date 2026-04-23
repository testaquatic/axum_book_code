use std::env;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn init_db() -> Pool<Postgres> {
    PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file"))
        .await
        .unwrap_or_else(|e| panic!("Error connecting to database: {}", e))
}
