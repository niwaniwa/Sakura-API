use super::super::database::models::NewAccountEntity;
use crate::domain::object::account::Account;
use crate::domain::repository::account::AccountRepository;
use anyhow::Result;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

impl NewAccountEntity {
    pub fn new(username: String, card_id: Vec<u8>) -> Self {
        Self { username, card_id }
    }

    fn from(model: Account) -> NewAccountEntity {
        NewAccountEntity {
            username: model.username.to_owned(),
            card_id: model.card_id.to_owned(),
        }
    }
}

pub struct AccountRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl AccountRepository for AccountRepositoryImpl {
    fn insert(&self, account: Account) -> Result<()> {
        use super::super::database::schema::account::dsl;

        let entity = NewAccountEntity::from(account);
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(dsl::account)
            .values(entity)
            .execute(&mut conn)?;

        Ok(())
    }
}
