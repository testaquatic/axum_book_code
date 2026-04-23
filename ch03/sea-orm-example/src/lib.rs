use std::collections::HashMap;

use axum::{Json, extract::Query};
use sea_orm::{ColumnTrait, Condition, Database, EntityTrait, QueryFilter};

use crate::entities::users::{Column, Entity, Model};

mod entities;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:25432/axum";

async fn get_user(Query(params): Query<HashMap<String, String>>) -> Json<Model> {
    let conn = Database::connect(DATABASE_URL).await.unwrap();
    let mut condition = params
        .get("id")
        .map(|id| Condition::any().add(Column::Id.eq(id.parse::<i32>().unwrap())))
        .unwrap_or(Condition::any());

    if let Some(username) = params.get("username") {
        condition = condition.add(Column::Username.eq(username));
    }

    let user = Entity::find()
        .filter(condition)
        .one(&conn)
        .await
        .unwrap()
        .unwrap();

    Json(user)
}

pub async fn run_app() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    let app = axum::Router::new().route("/users", axum::routing::get(get_user));
    axum::serve(listener, app).await.unwrap();
}
