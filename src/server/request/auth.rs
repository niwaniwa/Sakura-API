use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct AuthAccountRequest {
    pub username: String,
    pub grade: i32,
    pub expiration_date: NaiveDateTime,
    pub email: String,
    pub password: String,
}
