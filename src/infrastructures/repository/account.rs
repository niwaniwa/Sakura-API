use super::super::database::models::{AccountEntity, NewAccountEntity};
use crate::domain::object::account::{Account, AccountId};
use crate::domain::repository::account::AccountRepository;
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

impl NewAccountEntity {
    pub fn new(username: String, card_id: Vec<u8>, created_at: NaiveDateTime) -> Self {
        Self {
            username,
            card_id,
            created_at,
        }
    }

    fn from(model: &Account) -> NewAccountEntity {
        NewAccountEntity {
            username: model.username.to_owned(),
            card_id: model.card_id.to_owned(),
            created_at: model.created_at.to_owned(),
        }
    }
}

impl AccountEntity {
    pub fn of(&self) -> Account {
        Account {
            id: AccountId::new(self.id),
            username: self.username.to_owned(),
            card_id: self.card_id.to_owned(),
            created_at: self.created_at.to_owned(),
        }
    }
}

pub struct AccountRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl AccountRepository for AccountRepositoryImpl {
    fn insert(&self, account: &Account) -> Result<()> {
        use super::super::database::schema::account::dsl;

        let entity = NewAccountEntity::from(account);
        let mut conn = self.pool.get()?;
        diesel::insert_into(dsl::account)
            .values(entity)
            .execute(&mut conn)?;

        Ok(())
    }

    fn list(&self) -> Result<Vec<Account>> {
        use super::super::database::schema::account::dsl;

        let query = dsl::account.into_boxed();
        let mut conn = self.pool.get()?;
        let results: Vec<AccountEntity> = query.limit(100).load(&mut conn)?;

        Ok(results.into_iter().map(|e| e.of()).collect())
    }

    fn find_by_id(&self, account_id: &AccountId) -> Result<Account> {
        use super::super::database::schema::account::dsl;
        use super::super::database::schema::account::id;

        let mut conn = self.pool.get()?;
        let entity: AccountEntity = dsl::account
            .filter(id.eq(account_id.get()))
            .get_result(&mut conn)?;

        Ok(entity.of())
    }
}
