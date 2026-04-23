use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{
    api::users::AppError,
    db::{ProductModel, insert_product, select_product, update_product},
};

#[derive(Deserialize)]
pub struct UpsertModel {
    id: Option<i32>,
    title: Option<String>,
    price: Option<i32>,
    category: Option<String>,
}

#[derive(Serialize)]
pub struct Product {
    id: i32,
    title: String,
    price: i32,
    category: String,
}

impl From<ProductModel> for Product {
    fn from(value: ProductModel) -> Self {
        Product {
            id: value.id,
            title: value.title,
            price: value.price,
            category: value.category,
        }
    }
}

pub async fn get_product(
    State(conn): State<Pool<Postgres>>,
    Query(params): Query<UpsertModel>,
) -> Result<Json<Vec<Product>>, AppError> {
    let products = select_product(
        &conn,
        params.id,
        params.title,
        params.price,
        params.category,
    )
    .await
    .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database Error"))?
    .into_iter()
    .map(Product::from)
    .collect::<Vec<_>>();

    Ok(Json(products))
}

pub async fn post_product(
    State(conn): State<Pool<Postgres>>,
    Json(product): Json<UpsertModel>,
) -> Result<Json<Product>, AppError> {
    if let (Some(title), Some(price), Some(category)) =
        (product.title, product.price, product.category)
    {
        let product = insert_product(&conn, &title, price, &category)
            .await
            .map(Product::from)
            .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?;

        return Ok(Json(product));
    }

    Err(AppError::new(
        StatusCode::BAD_REQUEST,
        "title or price or category field is not set",
    ))
}

pub async fn put_product(
    State(executor): State<Pool<Postgres>>,
    Json(product): Json<UpsertModel>,
) -> Result<Json<Product>, AppError> {
    let Some(id) = product.id else {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "id field is not set",
        ));
    };

    match update_product(
        &executor,
        id,
        product.title.as_deref(),
        product.price,
        product.category.as_deref(),
    )
    .await
    {
        Ok(product) => {
            let product = Product::from(product);
            Ok(Json(product))
        }
        Err(sqlx::Error::RowNotFound) => {
            Err(AppError::new(StatusCode::NOT_FOUND, "Product not found"))
        }
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}
