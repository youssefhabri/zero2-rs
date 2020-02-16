use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub mod models;
pub mod schema;

pub struct Database {
    _pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn connect() -> Self {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(15)
            .build(manager)
            .expect("Failed to create connection pool");

        Database { _pool: pool }
    }

    fn _conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self._pool
            .clone()
            .get()
            .expect("Attempt to get connection timed out")
    }
}
