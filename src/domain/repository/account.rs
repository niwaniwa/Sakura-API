use super::super::object::account::{Account, AccountId};
use anyhow::Result;

pub trait AccountRepository {
    fn insert(&self, account: &Account) -> Result<()>;
    fn list(&self) -> Result<Vec<Account>>;
    fn find_by_id(&self, account_id: &AccountId) -> Result<Account>;
    fn delete(&self, account: &Account) -> Result<()>;
}
