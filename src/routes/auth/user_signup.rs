use actix_web::{
    post,
    web::{self, Json},
};
use serde::{Deserialize, Serialize};

use crate::{
    utils::{
        api::Response,
        db::{
            sessions::create_session,
            user::{save_user, username_exists, get_user_by_username},
        },
    },
    AppState,
};

#[derive(Deserialize)]
pub struct UserSignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserSignupResponse {
    pub user_id: String,
    pub username: String,
    pub token: String,
}

#[post("/auth/user/signup")]
pub async fn user_signup(
    body: web::Json<UserSignupRequest>,
    db: web::Data<AppState>,
) -> Json<Response<UserSignupResponse>> {
    if username_exists(&body.username, &db.db).await {
        return Json(Response {
            success: false,
            status: 409,
            message: "Username already exists".to_string(),
            data: None,
        });
    }

    save_user(&body.username, &body.password, &db.db).await;

    let user = get_user_by_username(&body.username, &db.db).await.unwrap();

    let token = create_session(user.id.clone(), &db.db).await;

    Json(Response {
        success: true,
        status: 201,
        message: "User created successfully".to_string(),
        data: Some(UserSignupResponse {
            user_id: user.id,
            username: user.username,
            token,
        }),
    })
}
