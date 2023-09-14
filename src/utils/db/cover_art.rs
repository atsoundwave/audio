use sqlx::{Pool, Postgres};

pub async fn save_cover_art(song_id: &str, image_url: &str, db: &Pool<Postgres>) -> i32 {
    let result = sqlx::query!(
        r#"
        INSERT INTO cover_art (song_id, image_url)
        VALUES ($1, $2)
        RETURNING id
        "#,
        song_id,
        image_url
    )
    .fetch_one(db)
    .await
    .unwrap();

    result.id
}

pub async fn cover_art_exists(song_id: &str, db: &Pool<Postgres>) -> bool {
    let result = sqlx::query!(
        r#"
        SELECT EXISTS(SELECT 1 FROM cover_art WHERE song_id = $1)
        "#,
        song_id
    )
    .fetch_one(db)
    .await
    .unwrap();

    result.exists.unwrap()
}

pub async fn get_cover_art(song_id: &str, db: &Pool<Postgres>) -> String {
    let result = sqlx::query!(
        r#"
        SELECT image_url FROM cover_art WHERE song_id = $1
        "#,
        song_id
    )
    .fetch_one(db)
    .await
    .unwrap();

    result.image_url
}