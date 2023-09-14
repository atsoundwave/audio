use sqlx::{Pool, Postgres};

use crate::utils::crypto::{random_id, generate_token};

pub async fn create_session(user_id: String, db: &Pool<Postgres>) -> (String, String) {
    let session_token = generate_token();
    let refresh_token = generate_token();

    sqlx::query!(
        r#"
        INSERT INTO sessions (id, user_id, session_token, refresh_token)
        VALUES ($1, $2, $3, $4)
        "#,
        random_id(),
        user_id,
        session_token,
        refresh_token
    )
    .execute(db)
    .await
    .unwrap();

    (session_token, refresh_token)
}

pub async fn session_exists(user_id: &String, refresh_token: &String, db: &Pool<Postgres>) -> bool {
    let session = sqlx::query!(
        r#"
        SELECT * FROM sessions
        WHERE user_id = $1 AND refresh_token = $2 AND expires_at > NOW()
        "#,
        user_id,
        refresh_token
    )
    .fetch_optional(db)
    .await
    .unwrap();

    match session {
        Some(_) => true,
        None => false,
    }
}