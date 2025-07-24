use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{
    entities::users,
    utils::{app_error::AppError, hash::verify_password, jwt::create_token},
};

#[derive(Serialize, Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<String, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user by username: {error:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error logging in, please try again later",
            )
        })?
        .ok_or(AppError::new(
            StatusCode::BAD_REQUEST,
            "incorrect username and/or password",
        ))?;

    if !verify_password(&request_user.password, &user.password)? {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "incorrect username and/or password",
        ));
    }

    create_token(user.username.clone())
}
