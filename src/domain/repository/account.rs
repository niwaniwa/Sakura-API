use super::super::object::account::Account;
use anyhow::Result;

pub trait AccountRepository {
    fn insert(&self, account: Account) -> Result<()>;
}
