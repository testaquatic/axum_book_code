use std::{env, sync::LazyLock};

use axum::{
    body::Body,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::utils::app_error::AppError;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    username: String,
}

static SECRET_KEY: LazyLock<String> =
    LazyLock::new(|| env::var("SECRET_KEY").expect("SECRET_KEY must be set"));

pub fn create_token(username: String) -> Result<String, AppError> {
    let now = chrono::Utc::now();
    let expires_at = now + Duration::hours(1);
    let exp = expires_at.timestamp() as usize;
    let claims = Claims { exp, username };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(SECRET_KEY.as_bytes());

    encode(&token_header, &claims, &key).map_err(|err| {
        error!("Error creating token: {:?}", err);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There wa an error, please try again later",
        )
    })
}

pub fn validate_token(token: &str) -> Result<Claims, AppError> {
    // `replace`보다 나을 것 같아서 변경했다.
    let binding = token.trim_start_matches("Bearer ");
    let key = DecodingKey::from_secret(SECRET_KEY.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    decode::<Claims>(binding, &key, &validation)
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!")
            }
            _ => {
                error!("Error validating token: {:?}", err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating token")
            }
        })
        .and_then(|decoded| {
            if chrono::Utc::now().timestamp() > decoded.claims.exp as i64 {
                Err(AppError::new(
                    StatusCode::UNAUTHORIZED,
                    "not authenticated!",
                ))
            } else {
                Ok(decoded.claims)
            }
        })
}

pub async fn authenticate(
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let token = headers
        .get("Authorization")
        .ok_or(AppError::new(
            StatusCode::UNAUTHORIZED,
            "not authenticated!",
        ))?
        .to_str()
        .map_err(|err| {
            error!("Error extracting token from headers: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
        })?;

    let claim = validate_token(token)?;

    debug!("Authenticated user: {}", claim.username);

    if claim.exp < (chrono::Utc::now().timestamp() as usize) {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "Token has expired"));
    }

    Ok(next.run(request).await)
}
