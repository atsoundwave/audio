#[derive(serde::Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub data: Option<T>,
}