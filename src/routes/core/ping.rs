use actix_web::{get, web::Json};
use crate::utils::api::Response;

#[derive(serde::Serialize)]
pub struct PingResponse {
    success: bool,
}

#[get("/core/ping")]
pub async fn ping() -> Json<Response<PingResponse>> {
    Json(Response {
        success: true,
        status: 200,
        message: "Pong".to_string(),
        data: None,
    })
}
