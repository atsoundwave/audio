use actix_web::{get, web::Json};

#[derive(serde::Serialize)]
pub struct PingResponse {
    success: bool,
}

#[get("/core/ping")]
pub async fn ping() -> Json<PingResponse> {
    Json(PingResponse { success: true })
}
