use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use sea_orm::{ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{entities::users, utils::app_error::AppError};

pub async fn get_user(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<users::Model>>, AppError> {
    let mut condition = Condition::all();

    if let Some(id) = params.get("id") {
        let parsed_id = id
            .parse::<i64>()
            .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "ID must be an integer"))?;
        condition = condition.add(users::Column::Id.eq(parsed_id));
    }
    if let Some(username) = params.get("username") {
        condition = condition.add(users::Column::Username.contains(username));
    }

    users::Entity::find()
        .filter(condition)
        .all(&conn)
        .await
        .map(Json)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}

#[derive(serde::Deserialize)]
pub struct UpsertModel {
    id: Option<i64>,
    username: Option<String>,
    password: Option<String>,
}

pub async fn post_user(
    State(conn): State<DatabaseConnection>,
    Json(user): Json<UpsertModel>,
) -> Result<Json<users::Model>, AppError> {
    let username = user.username.ok_or(AppError::new(
        StatusCode::BAD_REQUEST,
        "Username is required",
    ))?;

    let password = user.password.ok_or(AppError::new(
        StatusCode::BAD_REQUEST,
        "Password is required",
    ))?;

    let new_user = users::ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(username),
        password: ActiveValue::Set(password),
    };

    users::Entity::insert(new_user)
        .exec_with_returning(&conn)
        .await
        .map(Json)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}

pub async fn put_user(
    State(conn): State<DatabaseConnection>,
    Json(user): Json<UpsertModel>,
) -> Result<Json<users::Model>, AppError> {
    let Some(id) = user.id else {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "User ID not provided",
        ));
    };

    let found_user = users::Entity::find_by_id(id)
        .one(&conn)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .ok_or(AppError::new(StatusCode::NOT_FOUND, "User not found"))?;

    let mut active_user = users::ActiveModel::from(found_user);
    active_user.username = user
        .username
        .map(ActiveValue::Set)
        .unwrap_or(active_user.username);
    active_user.password = user
        .password
        .map(ActiveValue::Set)
        .unwrap_or(active_user.password);

    users::Entity::update(active_user)
        .exec(&conn)
        .await
        .map(Json)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}

pub async fn delete_user(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<&'static str>, AppError> {
    let id = params
        .get("id")
        .ok_or(AppError::new(
            StatusCode::BAD_REQUEST,
            "User ID not provided",
        ))?
        .parse::<i64>()
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "ID must be an integer"))?;

    users::Entity::delete_by_id(id)
        .exec(&conn)
        .await
        .map(|_| Json("User deleted"))
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}
