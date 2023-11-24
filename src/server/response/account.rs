use crate::domain::object::account::Account;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccountListResopnse {
    accounts: Vec<AccountDto>,
}

impl AccountListResopnse {
    pub fn new(accounts: Vec<Account>) -> AccountListResopnse {
        AccountListResopnse {
            accounts: accounts.iter().map(|a| AccountDto::new(&a)).collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AccountDto {
    id: i64,
    username: String,
    card_id: Vec<u8>,
    created_at: NaiveDateTime,
}

impl AccountDto {
    pub fn new(model: &Account) -> AccountDto {
        AccountDto {
            id: model.id.to_owned(),
            username: model.username.to_owned(),
            card_id: model.card_id.to_owned(),
            created_at: model.created_at.to_owned(),
        }
    }
}
