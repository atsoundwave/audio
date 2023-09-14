use actix_web::{
    get,
    web::{self, Json}, HttpRequest,
};
use serde::{Deserialize, Serialize};

use crate::{
    utils::{
        api::Response,
        db::{sessions::get_session_info, user::get_user_by_id},
    },
    AppState,
};

#[derive(Deserialize)]
pub struct SessionInfoRequest {
    pub session_token: String,
}

#[derive(Serialize)]
pub struct SessionInfoResponse {
    pub is_valid: bool,
    pub user_id: String,
    pub username: String,
}

#[get("/auth/session/info")]
pub async fn session_info(
    request: HttpRequest,
    db: web::Data<AppState>,
) -> Json<Response<SessionInfoResponse>> {
    // get session_token from Authorization header
    let session_token = match request.headers().get("Authorization") {
        Some(header) => match header.to_str() {
            Ok(header) => header.to_string(),
            Err(_) => {
                return Json(Response {
                    success: true,
                    status: 401,
                    message: "Session is invalid".to_string(),
                    data: None
                })
            }
        },
        None => {
            return Json(Response {
                success: true,
                status: 401,
                message: "No session token provided".to_string(),
                data: None
            })
        }
    };

    let session = match get_session_info(&session_token, &db.db).await {
        Some(session) => session,
        None => {
            return Json(Response {
                success: true,
                status: 401,
                message: "Session is invalid".to_string(),
                data: None
            })
        }
    };

    let user = get_user_by_id(&session.user_id, &db.db).await.unwrap();

    Json(Response {
        success: true,
        status: 200,
        message: "Session is valid".to_string(),
        data: Some(SessionInfoResponse {
            is_valid: true,
            user_id: session.user_id,
            username: user.username,
        }),
    })
}
