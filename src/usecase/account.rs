use crate::domain::object::account::Account;
use crate::domain::repository::account::AccountRepository;
use anyhow::Result;

pub fn post_account(repository: impl AccountRepository, account: Account) -> Result {
    repository.insert(account)
}
