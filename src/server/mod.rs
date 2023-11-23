mod handler;
mod request;
mod response;

use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

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
}
