use crate::domain::object::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AccountRequest {
    username: String,
}

impl AccountRequest {
    pub fn of(&self) -> Account {
        let card_id = Account::register_card_id();
        Account::create(self.username.to_owned(), card_id.to_owned())
    }
}
