use serenity::{model::id::GuildId, prelude::Context};
use std::str::FromStr;

use crate::core::consts::DB;

pub fn get_global_config_with_default<K, V>(context: &Context, key: K, default: V) -> V
where
    K: ToString,
    V: FromStr,
{
    get_global_config(context, key).unwrap_or(default)
}

pub fn get_global_config<K, V>(_context: &Context, key: K) -> Option<V>
where
    K: ToString,
    V: FromStr,
{
    let config = DB.get_config(key, None);

    config.ok()?.value.parse::<V>().ok()
}

pub fn get_guild_config_with_default<K, V>(
    context: &Context,
    guild_id: GuildId,
    key: K,
    default: V,
) -> V
where
    K: ToString,
    V: FromStr,
{
    get_guild_config(context, guild_id, key).unwrap_or(default)
}

pub fn get_guild_config<K, V>(_context: &Context, guild_id: GuildId, key: K) -> Option<V>
where
    K: ToString,
    V: FromStr,
{
    let config = DB.get_config(key, Some(*guild_id.as_u64())).ok()?;

    config.value.parse::<V>().ok()
}
