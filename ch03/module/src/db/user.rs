use sqlx::{Pool, Postgres, query_as};

use crate::db::UserModel;

/// DB에서 유저를 가져오는 함수
/// id와 username이 None이면 모든 유저를 가져온다.
pub async fn get_user_from_database(
    pool: &Pool<Postgres>,
    id: Option<i32>,
    username: Option<String>,
) -> Result<Vec<UserModel>, sqlx::Error> {
    let result = match (id, username) {
        // Querybuilder를 사용하는 방법도 있지만 가지가 많지 않으므로 직접 쿼리를 작성한다.
        (Some(id), Some(username)) => {
            query_as!(
                UserModel,
                "SELECT * FROM users WHERE id = $1 AND username = $2",
                id,
                username
            )
            .fetch_all(pool)
            .await?
        }
        (Some(id), None) => {
            query_as!(UserModel, "SELECT * FROM users WHERE id = $1", id)
                .fetch_all(pool)
                .await?
        }

        (None, Some(username)) => {
            query_as!(
                UserModel,
                "SELECT * FROM users WHERE username = $1",
                username
            )
            .fetch_all(pool)
            .await?
        }
        (None, None) => {
            query_as!(UserModel, "SELECT * FROM users")
                .fetch_all(pool)
                .await?
        }
    };

    Ok(result)
}

/// DB에 유저를 삽입하는 함수
pub async fn insert_user_to_database(
    pool: &Pool<Postgres>,
    username: &str,
    password: &str,
) -> Result<UserModel, sqlx::Error> {
    query_as!(
        UserModel,
        r#"INSERT INTO users (username, password) VALUES ($1, $2) RETURNING *"#,
        username,
        password
    )
    .fetch_one(pool)
    .await
}

/// DB에서 유저를 삭제하는 함수
pub async fn delete_user_from_database(
    pool: &Pool<Postgres>,
    id: i32,
) -> Result<UserModel, sqlx::Error> {
    query_as!(
        UserModel,
        r#"DELETE FROM users WHERE id = $1 RETURNING *"#,
        id
    )
    .fetch_one(pool)
    .await
}

/// DB에서 유저를 업데이트 하는 함수
/// username과 password가 모두 None이면 업데이트 하지 않고 기존 값을 반환한다.
pub async fn update_user_from_database(
    pool: &Pool<Postgres>,
    id: i32,
    username: Option<String>,
    password: Option<String>,
) -> Result<UserModel, sqlx::Error> {
    match (username, password) {
        (Some(username), Some(password)) => {
            query_as!(
                UserModel,
                r#"UPDATE users SET username = $1, password = $2 WHERE id = $3 RETURNING *"#,
                username,
                password,
                id
            )
            .fetch_one(pool)
            .await
        }
        (Some(username), None) => {
            query_as!(
                UserModel,
                r#"UPDATE users SET username = $1 WHERE id = $2 RETURNING *"#,
                username,
                id
            )
            .fetch_one(pool)
            .await
        }
        (None, Some(password)) => {
            query_as!(
                UserModel,
                r#"UPDATE users SET password = $1 WHERE id = $2 RETURNING *"#,
                password,
                id
            )
            .fetch_one(pool)
            .await
        }
        (None, None) => {
            query_as!(UserModel, r#"SELECT * FROM users WHERE id = $1"#, id)
                .fetch_one(pool)
                .await
        }
    }
}
