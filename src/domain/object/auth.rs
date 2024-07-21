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

    pub fn validate(email: &str, password: &str) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if !ValidateEmail::validate_email(&email) {
            errors.add("email", ValidationError::new("Invalid email format"));
        }
        if password.len() < 8 {
            errors.add(
                "password",
                ValidationError::new("Password must be at least 8 characters long"),
            );
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn hash_password(password: &str) -> Result<String, BcryptError> {
        hash(password, DEFAULT_COST)
    }
}
