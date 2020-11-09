use pickledb::{error::Error as PickleDdError, PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::{de::DeserializeOwned, Serialize};
use serenity::model::prelude::GuildId;
use serenity::prelude::{RwLock, TypeMapKey};
use std::sync::Arc;

pub struct Zero2ConfigContainer {
    db: PickleDb,
}

impl TypeMapKey for Zero2ConfigContainer {
    type Value = Arc<RwLock<Zero2ConfigContainer>>;
}

impl Zero2ConfigContainer {
    pub fn new() -> Zero2ConfigContainer {
        let db = PickleDb::load(
            "data/config.json",
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )
        .unwrap();
        Zero2ConfigContainer { db }
    }

    pub fn guild_config<K, V>(&self, guild_id: GuildId, config: K) -> Option<V>
    where
        K: ToString,
        V: DeserializeOwned,
    {
        let key = format!("guild:{}:{}", guild_id.as_u64(), config.to_string());
        self.db.get::<V>(&key)
    }

    pub fn set_guild_config<V>(
        &mut self,
        guild_id: GuildId,
        config: impl ToString,
        value: impl Serialize,
    ) -> Result<(), PickleDdError>
    where
        V: DeserializeOwned,
    {
        let key = format!("guild:{}:{}", guild_id.as_u64(), config.to_string());
        self.db.set(&key, &value)
    }

    pub fn global_config<K, V>(&self, config: K) -> Option<V>
    where
        K: ToString,
        V: DeserializeOwned,
    {
        let key = format!("global:{}", config.to_string());
        self.db.get::<V>(&key)
    }

    pub fn set_global_config(
        &mut self,
        config: impl ToString,
        value: impl Serialize,
    ) -> Result<(), PickleDdError> {
        let key = format!("global:{}", config.to_string());
        self.db.set(&key, &value)
    }
}
