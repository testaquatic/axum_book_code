use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::db::{
    UserModel, delete_user_from_database, get_user_from_database, insert_user_to_database,
    update_user_from_database,
};

/// 오류가 발생했을 때 반환하는 구조체
pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        AppError {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.code, Json(json!(self.message))).into_response()
    }
}

#[derive(Serialize)]
pub struct User {
    id: i32,
    username: String,
    password: String,
}

impl From<UserModel> for User {
    fn from(value: UserModel) -> Self {
        User {
            id: value.id,
            username: value.username,
            password: value.password,
        }
    }
}

/// "GET /users" 핸들러
pub async fn get_user(
    State(conn): State<Pool<Postgres>>,
    // 이 부분은 추후에 구조체로 변경하는 것이 좋을 것 같다.
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<User>>, AppError> {
    let Ok(id) = params.get("id").map(|s| s.parse::<i32>()).transpose() else {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "ID must be an integer",
        ));
    };
    let username = params.get("username").map(|s| s.to_string());

    let Ok(user_models) = get_user_from_database(&conn, id, username).await else {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        ));
    };

    Ok(Json(user_models.into_iter().map(User::from).collect()))
}

#[derive(Deserialize)]
pub struct UpsertModel {
    id: Option<i32>,
    username: Option<String>,
    password: Option<String>,
}

/// "POST /users" 핸들러
pub async fn post_user(
    State(conn): State<Pool<Postgres>>,
    Json(user): Json<UpsertModel>,
) -> Result<Json<User>, AppError> {
    if let Some(username) = user.username
        && let Some(password) = user.password
    {
        return insert_user_to_database(&conn, &username, &password)
            .await
            .map(|user_model| Json(user_model.into()))
            .map_err(|_| AppError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Database error".into(),
            });
    }

    Err(AppError {
        code: StatusCode::BAD_REQUEST,
        message: "Username or Password not provided".into(),
    })
}

/// "PUT /users" 핸들러
pub async fn put_user(
    State(conn): State<Pool<Postgres>>,
    Json(user): Json<UpsertModel>,
) -> Result<Json<User>, AppError> {
    let Some(id) = user.id else {
        return Err(AppError {
            code: StatusCode::BAD_REQUEST,
            message: "ID must be provided".into(),
        });
    };

    if user.password.is_none() && user.username.is_none() {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Username or Password must be provided",
        ));
    }

    update_user_from_database(&conn, id, user.username, user.password)
        .await
        .map(|user_model| Json(user_model.into()))
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => AppError::new(StatusCode::NOT_FOUND, "User not found"),
            _ => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database Error"),
        })
}

/// "DELETE /users" 핸들러
pub async fn delete_user(
    State(conn): State<Pool<Postgres>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<&'static str>, AppError> {
    let id_param = params.get("id").ok_or(AppError::new(
        StatusCode::BAD_REQUEST,
        "User Id not provided",
    ))?;
    let id = id_param
        .parse::<i32>()
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "User Id must be an integer"))?;
    delete_user_from_database(&conn, id)
        .await
        .map(|_| Json("User deleted"))
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}
