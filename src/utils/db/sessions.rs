use sqlx::{Pool, Postgres};

use crate::utils::crypto::{random_id, generate_token};

pub async fn create_session(user_id: String, db: &Pool<Postgres>) -> String {
    let token = generate_token();
    sqlx::query!(
        r#"
        INSERT INTO sessions (id, user_id, token)
        VALUES ($1, $2, $3)
        "#,
        random_id(),
        user_id,
        token,
    )
    .execute(db)
    .await
    .unwrap();

    token
}