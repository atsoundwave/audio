use crate::utils::crypto::{hash, random_id};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
}

pub struct FullUser {
    pub id: String,
    pub username: String,
    pub password: String,
}

pub async fn username_exists(username: &str, db: &Pool<Postgres>) -> bool {
    let result = sqlx::query!(
        r#"SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)"#,
        username
    )
    .fetch_one(db)
    .await
    .unwrap();

    result.exists.unwrap()
}

pub async fn get_user_by_username(username: &str, db: &Pool<Postgres>) -> Option<User> {
    let result = sqlx::query_as!(
        User,
        r#"SELECT id, username FROM users WHERE username = $1"#,
        username
    )
    .fetch_one(db)
    .await;

    match result {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

pub async fn get_full_user_by_username(
    username: &str,
    db: &Pool<Postgres>,
) -> Option<FullUser> {
    let result = sqlx::query_as!(
        FullUser,
        r#"SELECT id, username, password FROM users WHERE username = $1"#,
        username
    )
    .fetch_one(db)
    .await;

    match result {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

pub async fn save_user(username: &str, password: &str, db: &Pool<Postgres>) {
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, password)
        VALUES ($1, $2, $3)
        "#,
        random_id(),
        username,
        hash(password)
    )
    .execute(db)
    .await
    .unwrap();
}
