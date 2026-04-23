use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{
    api::users::AppError,
    db::{
        delete_category_from_database, get_all_categories_from_database,
        get_categories_by_name_from_database, insert_category_to_database,
    },
};

#[derive(Serialize, Deserialize)]
pub struct Category {
    name: String,
}

/// GET category 핸들러
/// 쿼리를 받고 카테고리 목록을 반환한다.
pub async fn get_category(
    State(conn): State<Pool<Postgres>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Category>>, AppError> {
    let result = match params.get("name") {
        Some(name) => get_categories_by_name_from_database(&conn, name).await,
        None => get_all_categories_from_database(&conn).await,
    };

    result
        .map(|categories| {
            let categories = categories
                .into_iter()
                .map(|cat| Category { name: cat.name })
                .collect();
            Json(categories)
        })
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}

/// POST category 핸들러
/// 카테고리를 생성한다.
pub async fn post_category(
    State(conn): State<Pool<Postgres>>,
    Json(category): Json<Category>,
) -> Result<Json<Category>, AppError> {
    insert_category_to_database(&conn, &category.name)
        .await
        .map(|category| {
            Json(Category {
                name: category.name,
            })
        })
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}

/// DELETE category 핸들러
/// 카테고리를 삭제한다.
pub async fn delete_category(
    State(conn): State<Pool<Postgres>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<&'static str>, AppError> {
    let Some(name) = params.get("name") else {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Category name not provided",
        ));
    };

    match delete_category_from_database(&conn, name).await {
        Ok(_) => Ok(Json("Deleted")),
        Err(sqlx::Error::RowNotFound) => {
            Err(AppError::new(StatusCode::NOT_FOUND, "Category not found"))
        }
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}
