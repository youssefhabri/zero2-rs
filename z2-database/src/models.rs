use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};

use super::schema::configs;

#[derive(Debug, Queryable, SimpleObject)]
pub struct Config {
    pub id: i32,
    pub name: String,
    pub value: String,
    pub guild_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "configs"]
pub struct NewConfig {
    pub name: String,
    pub value: String,
    pub guild_id: i64,
}
