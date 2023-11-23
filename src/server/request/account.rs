use crate::domain::object::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AccountRequest {
    username: String,
}

impl AccountRequest {
    pub fn of(&self) -> Account {
        Account::create(self.username.to_owned())
    }
}
