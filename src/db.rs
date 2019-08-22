use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Deref;

pub mod models;
pub mod schema;

use self::schema::*;

pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn connect() -> Self {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL");
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

    pub fn blacklist(&self, type_: &str) -> QueryResult<HashSet<String>> {
        let blacklist = blacklist::table
            .load::<(i32, String, Option<DateTime<Utc>>, String)>(self.conn().deref())?
            .iter()
            .filter(|(_, b_type, _, _)| b_type == type_)
            .map(|(_, _, _, value)| value.clone())
            .collect::<HashSet<String>>();

        Ok(blacklist)
    }
}
