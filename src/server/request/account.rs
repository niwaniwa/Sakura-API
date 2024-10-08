use crate::domain::object::account::{Account, AccountId};
use crate::domain::object::auth::AuthId;
use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct AccountRequest {
    pub auth_id: AuthId,
    pub username: String,
    pub grade: i32,
    pub expiration_date: NaiveDateTime,
}

#[derive(Debug, Default, Deserialize)]
pub struct AccountIdRequest {
    pub account_id: i64,
}

impl AccountRequest {
    pub fn of(&self) -> Account {
        Account::new(
            self.auth_id.to_owned(),
            self.username.to_owned(),
            self.grade.to_owned(),
            self.expiration_date.to_owned(),
        )
    }
    pub fn model(&self, account_id: AccountId, created_at: NaiveDateTime) -> Account {
        Account {
            id: account_id,
            auth_id: self.auth_id.to_owned(),
            username: self.username.to_owned(),
            grade: self.grade.to_owned(),
            expiration_date: self.expiration_date.to_owned(),
            created_at,
        }
    }
}
