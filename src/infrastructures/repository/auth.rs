use super::super::database::models::{AuthEntity, NewAuthEntity};
use crate::domain::object::auth::{Auth, AuthId};
use crate::domain::repository::auth::AuthRepository;
use anyhow;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

impl NewAuthEntity {
    pub fn new(email: String, password: String, created_at: NaiveDateTime) -> Self {
        Self {
            email,
            password,
            created_at,
        }
    }

    fn from(model: &Auth) -> NewAuthEntity {
        NewAuthEntity {
            email: model.email.to_owned(),
            password: model.password.to_owned(),
            created_at: model.created_at.to_owned(),
        }
    }
}

impl AuthEntity {
    fn from(model: &Auth) -> AuthEntity {
        AuthEntity {
            id: model.id.get(),
            email: model.email.to_owned(),
            password: model.password.to_owned(),
            created_at: model.created_at.to_owned(),
        }
    }

    fn of(&self) -> Auth {
        Auth {
            id: AuthId::new(self.id),
            email: self.email.to_owned(),
            password: self.password.to_owned(),
            created_at: self.created_at.to_owned(),
        }
    }
}

pub struct AuthRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl AuthRepository for AuthRepositoryImpl {
    // fn signup(&self, auth: &Auth) -> anyhow::Result<()> {
    //     use super::super::database::schema::auth::dsl;

    //     let entity = NewAuthEntity::from(auth);
    //     let mut conn = self.pool.get()?;
    //     diesel::

    // }
    fn exist_by_email(&self, email: &str) -> anyhow::Result<bool> {
        use super::super::database::schema::auth::dsl;

        let mut conn = self.pool.get()?;
        let query = dsl::auth.filter(dsl::email.eq(email));

        match query.first::<AuthEntity>(&mut conn) {
            Ok(_) => Ok(true),
            Err(diesel::result::Error::NotFound) => Ok(false),
            Err(err) => Err(anyhow::Error::new(err)),
        }
    }
}
