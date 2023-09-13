use sqlx::{Pool, Postgres};

pub async fn setup_tables(db: &Pool<Postgres>) {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
        )
        "#
    )
    .execute(db)
    .await
    .unwrap();

    log::info!("Created table users");

    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            token TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            expires_at TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '1 day',
            FOREIGN KEY (user_id) REFERENCES users (id)
        )
        "#
    )
    .execute(db)
    .await
    .unwrap();

    log::info!("Created table sessions");
}