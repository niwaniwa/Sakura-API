use super::super::database::models::{AccountEntity, NewAccountEntity};
use crate::domain::object::account::{Account, AccountId};
use crate::domain::object::auth::AuthId;
use crate::domain::repository::account::AccountRepository;
use anyhow;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

impl NewAccountEntity {
    pub fn new(
        auth_id: AuthId,
        username: String,
        grade: i32,
        expiration_date: NaiveDateTime,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            auth_id: auth_id.get().to_owned(),
            username,
            grade,
            expiration_date,
            created_at,
        }
    }

    fn from(model: &Account) -> NewAccountEntity {
        NewAccountEntity {
            auth_id: model.auth_id.get().to_owned(),
            username: model.username.to_owned(),
            grade: model.grade.to_owned(),
            expiration_date: model.expiration_date.to_owned(),
            created_at: model.created_at.to_owned(),
        }
    }
}

impl AccountEntity {
    fn from(model: &Account) -> AccountEntity {
        AccountEntity {
            id: model.id.get(),
            auth_id: model.auth_id.get().to_owned(),
            username: model.username.to_owned(),
            grade: model.grade.to_owned(),
            expiration_date: model.expiration_date.to_owned(),
            created_at: model.created_at.to_owned(),
        }
    }
    fn of(&self) -> Account {
        Account {
            id: AccountId::new(self.id),
            auth_id: AuthId::new(self.auth_id),
            username: self.username.to_owned(),
            grade: self.grade.to_owned(),
            expiration_date: self.expiration_date.to_owned(),
            created_at: self.created_at.to_owned(),
        }
    }
}

pub struct AccountRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl AccountRepository for AccountRepositoryImpl {
    fn insert(&self, account: &Account) -> anyhow::Result<()> {
        use super::super::database::schema::account::dsl;

        let entity = NewAccountEntity::from(account);
        let mut conn = self.pool.get()?;
        diesel::insert_into(dsl::account)
            .values(entity)
            .execute(&mut conn)?;

        Ok(())
    }

    fn list(&self) -> anyhow::Result<Vec<Account>> {
        use super::super::database::schema::account::dsl;

        let query = dsl::account.into_boxed();
        let mut conn = self.pool.get()?;
        let results: Vec<AccountEntity> = query.limit(100).load(&mut conn)?;

        Ok(results.into_iter().map(|e| e.of()).collect())
    }

    fn find_by_id(&self, account_id: &AccountId) -> anyhow::Result<Account> {
        use super::super::database::schema::account::dsl;
        use super::super::database::schema::account::id;

        let mut conn = self.pool.get()?;
        let entity: AccountEntity = dsl::account
            .filter(id.eq(account_id.get()))
            .get_result(&mut conn)?;

        Ok(entity.of())
    }

    fn update(&self, account: &Account) -> anyhow::Result<()> {
        use super::super::database::schema::account::dsl;

        let mut conn = self.pool.get()?;
        let entity = AccountEntity::from(account);
        diesel::update(dsl::account.filter(dsl::id.eq(account.id.get())))
            .set(&entity)
            .execute(&mut conn)?;

        Ok(())
    }

    fn delete(&self, account: &Account) -> anyhow::Result<()> {
        use super::super::database::schema::account::dsl as account_dsl;
        use super::super::database::schema::card::dsl as card_dsl;

        let entity = AccountEntity::from(account);
        let mut conn = self.pool.get()?;

        conn.transaction::<_, anyhow::Error, _>(|conn| {
            diesel::delete(card_dsl::card.filter(card_dsl::account_id.eq(entity.id)))
                .execute(conn)?;

            diesel::delete(account_dsl::account.filter(account_dsl::id.eq(entity.id)))
                .execute(conn)?;

            Ok(())
        })
    }
}
