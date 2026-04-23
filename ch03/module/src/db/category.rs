use sqlx::{pool, query_as};

use crate::db::model::CategoryModel;

/// 카테고리의 목록을 데이터베이스에서 가져온다.
pub async fn get_all_categories_from_database(
    pool: &pool::Pool<sqlx::Postgres>,
) -> Result<Vec<CategoryModel>, sqlx::Error> {
    query_as!(CategoryModel, "SELECT * FROM category")
        .fetch_all(pool)
        .await
}

/// 특정 문자열이 포함된 카테고리를 데이터베이스에서 가져온다.
pub async fn get_categories_by_name_from_database(
    pool: &pool::Pool<sqlx::Postgres>,
    name: &str,
) -> Result<Vec<CategoryModel>, sqlx::Error> {
    query_as!(
        CategoryModel,
        "SELECT * FROM category WHERE name ILIKE $1",
        name
    )
    .fetch_all(pool)
    .await
}

/// 카테고리를 데이터베이스에 삽입한다.
pub async fn insert_category_to_database(
    pool: &pool::Pool<sqlx::Postgres>,
    name: &str,
) -> Result<CategoryModel, sqlx::Error> {
    query_as!(
        CategoryModel,
        "INSERT INTO category (name) VALUES ($1) RETURNING *",
        name
    )
    .fetch_one(pool)
    .await
}

/// 데이터베이스에서 카테고리를 삭제한다.
pub async fn delete_category_from_database(
    pool: &pool::Pool<sqlx::Postgres>,
    name: &str,
) -> Result<CategoryModel, sqlx::Error> {
    query_as!(
        CategoryModel,
        "DELETE FROM category WHERE name = $1 RETURNING *",
        name
    )
    .fetch_one(pool)
    .await
}
