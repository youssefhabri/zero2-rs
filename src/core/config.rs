use serenity::{model::id::GuildId, prelude::Context};
use std::str::FromStr;

#[cfg(feature = "db")]
use crate::core::consts::DB;

pub fn get_global_config_with_default<K, V>(_context: &Context, _key: K, default: V) -> V
where
    K: ToString,
    V: FromStr,
{
    #[cfg(feature = "db")]
    return get_global_config(_context, _key).unwrap_or(default);

    #[cfg(not(feature = "db"))]
    return default;
}

#[cfg(feature = "db")]
pub fn get_global_config<K, V>(_context: &Context, key: K) -> Option<V>
where
    K: ToString,
    V: FromStr,
{
    let config = DB.get_config(key, None);

    config.ok()?.value.parse::<V>().ok()
}

pub fn get_guild_config_with_default<K, V>(
    _context: &Context,
    _guild_id: GuildId,
    _key: K,
    default: V,
) -> V
where
    K: ToString,
    V: FromStr,
{
    #[cfg(feature = "db")]
    return get_guild_config(_context, _guild_id, _key).unwrap_or(default);

    #[cfg(not(feature = "db"))]
    return default;
}

#[cfg(feature = "db")]
pub fn get_guild_config<K, V>(_context: &Context, guild_id: GuildId, key: K) -> Option<V>
where
    K: ToString,
    V: FromStr,
{
    let config = DB.get_config(key, Some(*guild_id.as_u64())).ok()?;

    config.value.parse::<V>().ok()
}
