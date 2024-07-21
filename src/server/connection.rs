use std::env;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;

use crate::domain::repository::account::AccountRepository;
use crate::domain::repository::auth::AuthRepository;
use crate::domain::repository::card::CardRepository;
use crate::domain::repository::door::DoorRepository;
use crate::domain::repository::register::RegisterRepository;
use crate::infrastructures::repository::account::AccountRepositoryImpl;
use crate::infrastructures::repository::auth::AuthRepositoryImpl;
use crate::infrastructures::repository::card::CardRepositoryImpl;
use crate::infrastructures::repository::door::DoorRepositoryImpl;
use crate::infrastructures::repository::register::RegisterRepositoryImpl;

pub struct RequestContext {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl RequestContext {
    pub fn new() -> RequestContext {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL i not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create DB connection pool.");

        RequestContext { pool }
    }

    pub fn account_repository(&self) -> impl AccountRepository {
        AccountRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }
    pub fn card_repository(&self) -> impl CardRepository {
        CardRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }

    pub fn door_repository(&self) -> impl DoorRepository {
        DoorRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }

    pub fn register_repository(&self) -> impl RegisterRepository {
        RegisterRepositoryImpl {}
    }

    pub fn auth_repository(&self) -> impl AuthRepository {
        AuthRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }
}
