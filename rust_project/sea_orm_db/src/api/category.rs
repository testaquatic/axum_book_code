use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use sea_orm::{
    ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter,
};

use crate::{entities::category, utils::app_error::AppError};

pub async fn get_category(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<category::Model>>, AppError> {
    let mut condition = Condition::all();
    if let Some(name) = params.get("name") {
        condition = condition.add(category::Column::Name.contains(name));
    }

    category::Entity::find()
        .filter(condition)
        .all(&conn)
        .await
        .map(Json)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database Error"))
}

pub async fn post_category(
    State(conn): State<DatabaseConnection>,
    Json(category): Json<category::Model>,
) -> Result<Json<category::Model>, AppError> {
    let new_category = category::ActiveModel {
        name: ActiveValue::Set(category.name),
    };

    category::Entity::insert(new_category)
        .exec_with_returning(&conn)
        .await
        .map(Json)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database Error"))
}

pub async fn delete_category(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<&'static str>, AppError> {
    let Some(name) = params.get("name") else {
        return Err(AppError::new(StatusCode::BAD_REQUEST, "Name is required"));
    };

    let category_model = category::Entity::find()
        .filter(Condition::any().add(category::Column::Name.contains(name)))
        .one(&conn)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database Error"))?
        .ok_or(AppError::new(StatusCode::NOT_FOUND, "Category not found"))?;

    category::Entity::delete(category_model.into_active_model())
        .exec(&conn)
        .await
        .map(|_| Json("Category deleted"))
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}
