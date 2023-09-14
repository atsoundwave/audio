use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres, types::chrono::{DateTime, Utc}};

use crate::utils::crypto::{random_id, generate_token};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub session_token: String,
    pub refresh_token: String,
}

pub struct FullSession {
    pub id: String,
    pub user_id: String,
    pub session_token: String,
    pub refresh_token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

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

pub async fn get_session_info(session_token: &String, db: &Pool<Postgres>) -> Option<Session> {
    let session = sqlx::query_as!(
        FullSession,
        r#"
        SELECT * FROM sessions
        WHERE session_token = $1 AND expires_at > NOW()
        "#,
        session_token
    )
    .fetch_one(db)
    .await
    .ok()?;

    Some(Session {
        id: session.id,
        user_id: session.user_id,
        session_token: session.session_token,
        refresh_token: session.refresh_token,
    })
}