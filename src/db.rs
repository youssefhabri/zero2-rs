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

    pub fn commands_blacklist(&self) -> QueryResult<HashSet<String>> {
        let blacklist = commands_blacklist::table
            .load::<(i32, String, bool)>(self.conn().deref())?
            .iter()
            .filter(|(_, _, blocked)| *blocked)
            .map(|(_, cmd, _)| {
                let c = cmd.clone();
                c
            })
            .collect::<HashSet<String>>();

        Ok(blacklist)
    }
}
