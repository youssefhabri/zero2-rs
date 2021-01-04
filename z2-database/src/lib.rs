#[macro_use]
extern crate diesel;

mod config;
pub mod models;
mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Config error: {0}")]
    Config(config::ConfigError),
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;

pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn connect() -> Self {
        let database_url = kankyo::key("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(15)
            .build(manager)
            .expect("Failed to create connection pool");

        Database { pool }
    }

    fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool
            .clone()
            .get()
            .expect("Attempt to get connection timed out")
    }
}
