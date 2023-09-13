pub struct UserSignupRequest {
    pub username: String,
    pub password: String,
}

pub struct UserSignupResponse {
    pub user_id: i32,
    pub username: String,
    pub token: String,
}

#[post("/auth/user/signup")]
pub async fn user_signup(
    body: web::Json<UserSignupRequest>,
) -> Json<Response> {
    
}