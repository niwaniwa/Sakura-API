use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}
