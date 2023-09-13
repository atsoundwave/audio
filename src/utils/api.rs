#[derive(serde::Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}