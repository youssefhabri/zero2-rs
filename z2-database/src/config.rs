use chrono::Utc;
use diesel::prelude::*;

use crate::models::{Config, NewConfig};
use crate::{schema, Database, DatabaseError, DatabaseResult};

impl Database {
    pub fn all_configs(&self, guild_id: Option<u64>) -> DatabaseResult<Vec<Config>> {
        use schema::configs::dsl;

        let result = match guild_id {
            Some(guild_id) => dsl::configs
                .filter(dsl::guild_id.eq(guild_id as i64))
                .load::<Config>(&self.conn()),
            None => dsl::configs.load::<Config>(&self.conn()),
        };

        result.map_err(DatabaseError::AllConfig)
    }

    pub fn get_config(&self, name: impl ToString, guild_id: Option<u64>) -> DatabaseResult<Config> {
        use schema::configs::dsl;

        let guild_id = guild_id.map_or(0, |guild_id| guild_id as i64);
        dsl::configs
            .filter(dsl::name.eq(name.to_string()))
            .filter(dsl::guild_id.eq(guild_id))
            .first(&self.conn())
            .map_err(DatabaseError::GetConfig)
    }

    pub fn set_config(
        &self,
        name: impl ToString,
        value: impl ToString,
        guild_id: Option<u64>,
    ) -> DatabaseResult<Config> {
        use schema::configs::{self, dsl};

        let guild_id = guild_id.map_or(0, |guild_id| guild_id as i64);
        let new_config = NewConfig {
            name: name.to_string(),
            value: value.to_string(),
            guild_id,
        };

        diesel::insert_into(configs::table)
            .values(&new_config)
            .on_conflict((dsl::name, dsl::guild_id))
            .do_update()
            .set((
                dsl::value.eq(value.to_string()),
                dsl::updated_at.eq(Utc::now()),
            ))
            .get_result(&self.conn())
            .map_err(DatabaseError::SetConfig)
    }
}
