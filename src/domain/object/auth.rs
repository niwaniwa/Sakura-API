use super::Id;
use crate::utils::time::create_time;
use bcrypt::{hash, BcryptError, DEFAULT_COST};
use chrono::NaiveDateTime;
use validator::{ValidateEmail, ValidationError, ValidationErrors};

pub type AuthId = Id<Auth>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Auth {
    pub id: AuthId,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

impl Auth {
    pub fn new(email: String, password: String) -> Self {
        Self {
            id: Default::default(),
            email,
            password,
            created_at: create_time(),
        }
    }
}
