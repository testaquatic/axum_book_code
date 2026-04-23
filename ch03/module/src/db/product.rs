use sqlx::{Pool, Postgres, QueryBuilder, Row, postgres::PgQueryResult, query, query_as};

use crate::db::model::ProductModel;

pub async fn select_product(
    executor: &Pool<Postgres>,
    id: Option<i32>,
    title: Option<String>,
    price: Option<i32>,
    category: Option<String>,
) -> Result<Vec<ProductModel>, sqlx::Error> {
    let mut query_builder = QueryBuilder::<Postgres>::new("SELECT * from product");

    if id.is_some() || title.is_some() || price.is_some() || category.is_some() {
        query_builder.push(" WHERE ");
    }

    let mut first = true;

    let mut add_condition = |column: &str, value: String| {
        if first {
            first = false;
        } else {
            query_builder.push(" AND ");
        }
        query_builder
            .push(format_args!("{column} = "))
            .push_bind(value);
    };

    if let Some(id) = id {
        add_condition("id", id.to_string())
    }
    if let Some(title) = title {
        add_condition("title", title);
    }
    if let Some(price) = price {
        add_condition("price", price.to_string());
    }
    if let Some(category) = category {
        add_condition("category", category);
    }

    let product_query = query_builder.build();

    let products = product_query.fetch_all(executor).await?;

    products
        .into_iter()
        .map(|product| -> Result<ProductModel, sqlx::Error> {
            let product_model = ProductModel {
                id: product.try_get("id")?,
                title: product.try_get("title")?,
                price: product.try_get("price")?,
                category: product.try_get("category")?,
            };

            Ok(product_model)
        })
        .collect()
}

pub async fn insert_product(
    executor: &Pool<Postgres>,
    title: &str,
    price: i32,
    category: &str,
) -> Result<ProductModel, sqlx::Error> {
    let product_model = query_as!(
        ProductModel,
        "INSERT INTO product (title, price, category) VALUES ($1, $2, $3) RETURNING *",
        title,
        price,
        category
    )
    .fetch_one(executor)
    .await?;

    Ok(product_model)
}

pub async fn update_product(
    executor: &Pool<Postgres>,
    id: i32,
    title: Option<&str>,
    price: Option<i32>,
    category: Option<&str>,
) -> Result<ProductModel, sqlx::Error> {
    let product_id = query_as!(ProductModel, "SELECT * FROM product WHERE id = $1", id)
        .fetch_one(executor)
        .await?;

    let title = title.unwrap_or(&product_id.title);
    let price = price.unwrap_or(product_id.price);
    let category = category.unwrap_or(&product_id.category);

    let product_model = query_as!(
        ProductModel,
        "UPDATE product SET title = $1, price = $2, category = $3 WHERE id = $4 RETURNING *",
        title,
        price,
        category,
        id
    )
    .fetch_one(executor)
    .await?;

    Ok(product_model)
}

pub async fn delete_product(
    executor: &Pool<Postgres>,
    id: i32,
) -> Result<PgQueryResult, sqlx::Error> {
    query!("DELETE FROM product WHERE id = $1", id)
        .execute(executor)
        .await
}
