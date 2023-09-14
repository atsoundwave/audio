use actix_web::{post, web::{Json, self}};
use serde::{Serialize, Deserialize};

use crate::{utils::{api::Response, db::sessions::{session_exists, create_session}}, AppState};

#[derive(Deserialize)]
pub struct SessionRefreshRequest {
    pub user_id: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct SessionRefreshResponse {
    pub session_token: String,
    pub refresh_token: String,
}

#[post("/auth/session/refresh")]
pub async fn session_refresh(
    body: web::Json<SessionRefreshRequest>,
    db: web::Data<AppState>,
) -> Json<Response<SessionRefreshResponse>> {
    if !session_exists(&body.user_id, &body.refresh_token, &db.db).await {
        return Json(Response {
            success: false,
            status: 401,
            message: "Invalid session token".to_string(),
            data: None,
        });
    }

    let (session, refresh) = create_session(body.user_id.clone(), &db.db).await;

    Json(Response {
        success: true,
        status: 200,
        message: "Session refreshed successfully".to_string(),
        data: Some(SessionRefreshResponse {
            session_token: session,
            refresh_token: refresh,
        }),
    })
}