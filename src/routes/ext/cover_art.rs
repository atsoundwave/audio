use actix_web::{web::{self, Json}, get};
use serde::{Serialize, Deserialize};

use crate::{AppState, utils::{api::Response, db::cover_art::{cover_art_exists, get_cover_art, save_cover_art}, yt::get_yt_cover}};

#[derive(Serialize)]
pub struct CoverArtResponse {
    pub image_url: String,
}

#[derive(Deserialize)]
pub struct CoverArtRequestPath {
    pub song_id: String,
    pub artist: String,
    pub title: String,
}

#[get("/ext/cover/{song_id}/{artist}/{title}")]
pub async fn cover_art(
    path: web::Path<CoverArtRequestPath>,
    db: web::Data<AppState>,
) -> Json<Response<CoverArtResponse>> {
    if cover_art_exists(&path.song_id, &db.db).await {
        let image_url = get_cover_art(&path.song_id, &db.db).await;

        return Json(Response {
            success: true,
            status: 200,
            message: "Cover art found".to_string(),
            data: Some(CoverArtResponse {
                image_url,
            }),
        });
    }

    let image_url = get_yt_cover(&path.artist, &path.title).await;

    match image_url {
        Some(image_url) => {
            save_cover_art(&path.song_id, &image_url, &db.db).await;

            return Json(Response {
                success: true,
                status: 200,
                message: "Cover art found".to_string(),
                data: Some(CoverArtResponse {
                    image_url,
                }),
            });
        }
        None => {
            return Json(Response {
                success: false,
                status: 404,
                message: "Cover art not found".to_string(),
                data: None,
            });
        }
    }
}