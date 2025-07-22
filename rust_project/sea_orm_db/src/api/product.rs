use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use sea_orm::{
    ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter,
};

use crate::{entities::product, utils::app_error::AppError};

#[derive(serde::Deserialize)]
pub struct UpsertModel {
    id: Option<i64>,
    title: Option<String>,
    price: Option<i32>,
    category: Option<String>,
}

pub async fn get_product(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<UpsertModel>,
) -> Result<Json<Vec<product::Model>>, AppError> {
    let mut condition = Condition::all();

    if let Some(id) = params.id {
        condition = condition.add(product::Column::Id.eq(id));
    }
    if let Some(title) = params.title {
        condition = condition.add(product::Column::Title.contains(title));
    }
    if let Some(price) = params.price {
        condition = condition.add(product::Column::Price.eq(price));
    }
    if let Some(category) = params.category {
        condition = condition.add(product::Column::Category.contains(category));
    }

    product::Entity::find()
        .filter(condition)
        .all(&conn)
        .await
        .map(Json)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}

pub async fn post_product(
    State(conn): State<DatabaseConnection>,
    Json(product): Json<UpsertModel>,
) -> Result<Json<product::Model>, AppError> {
    let title = product
        .title
        .ok_or(AppError::new(StatusCode::BAD_REQUEST, "Title is required"))?;
    let price = product
        .price
        .ok_or(AppError::new(StatusCode::BAD_REQUEST, "Price is required"))?;
    let category = product.category.ok_or(AppError::new(
        StatusCode::BAD_REQUEST,
        "Category is required",
    ))?;

    let new_product = product::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(title),
        price: ActiveValue::Set(price),
        category: ActiveValue::Set(category),
    };

    product::Entity::insert(new_product)
        .exec_with_returning(&conn)
        .await
        .map(Json)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}

pub async fn put_product(
    State(conn): State<DatabaseConnection>,
    Json(product): Json<UpsertModel>,
) -> Result<Json<product::Model>, AppError> {
    let id = product.id.ok_or(AppError::new(
        StatusCode::BAD_REQUEST,
        "Product ID not provided",
    ))?;
    let result = product::Entity::find_by_id(id)
        .one(&conn)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .ok_or(AppError::new(StatusCode::NOT_FOUND, "Product not found"))?;

    let new_product = product::ActiveModel {
        id: ActiveValue::Set(id),
        title: ActiveValue::Set(product.title.unwrap_or(result.title)),
        price: ActiveValue::Set(product.price.unwrap_or(result.price)),
        category: ActiveValue::Set(product.category.unwrap_or(result.category)),
    };

    product::Entity::update(new_product)
        .exec(&conn)
        .await
        .map(Json)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}

pub async fn delete_product(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<UpsertModel>,
) -> Result<Json<&'static str>, AppError> {
    let mut condition = Condition::any();

    if let Some(id) = params.id {
        condition = condition.add(product::Column::Id.eq(id));
    }
    if let Some(title) = params.title {
        condition = condition.add(product::Column::Title.contains(title));
    }
    if let Some(price) = params.price {
        condition = condition.add(product::Column::Price.eq(price));
    }
    if let Some(category) = params.category {
        condition = condition.add(product::Column::Category.contains(category));
    }

    let product = product::Entity::find()
        .filter(condition)
        .one(&conn)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .ok_or(AppError::new(StatusCode::NOT_FOUND, "Product not found"))?;

    product::Entity::delete(product.into_active_model())
        .exec(&conn)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        .map(|_| Json("deleted"))
}
