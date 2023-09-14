use actix_web::{post, web::{self, Json}};
use serde::{Serialize, Deserialize};

use crate::{utils::{api::Response, db::{user::{username_exists, get_full_user_by_username}, sessions::create_session}, crypto::verify}, AppState};

#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserLoginResponse {
    pub user_id: String,
    pub username: String,
    pub session_token: String,
    pub refresh_token: String,
}

#[post("/auth/user/login")]
pub async fn user_login(
    body: web::Json<UserLoginRequest>,
    db: web::Data<AppState>,
) -> Json<Response<UserLoginResponse>> {
    if !username_exists(&body.username, &db.db).await {
        return Json(Response {
            success: false,
            status: 404,
            message: "Username not found".to_string(),
            data: None,
        });
    }

    let user = get_full_user_by_username(&body.username, &db.db).await.unwrap();

    if !verify(&body.password, &user.password) {
        return Json(Response {
            success: false,
            status: 401,
            message: "Incorrect password".to_string(),
            data: None,
        });
    }

    let (session, refresh) = create_session(user.id.clone(), &db.db).await;

    Json(Response {
        success: true,
        status: 200,
        message: "User logged in successfully".to_string(),
        data: Some(UserLoginResponse {
            user_id: user.id,
            username: user.username,
            session_token: session,
            refresh_token: refresh,
        }),
    })
}