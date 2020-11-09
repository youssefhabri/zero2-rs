use crate::core::config::Zero2ConfigContainer;
use serde::de::DeserializeOwned;
use serenity::{model::id::GuildId, prelude::Context};

pub async fn get_global_config_with_default<K, V>(context: &Context, key: K, default: V) -> V
where
    K: ToString,
    V: DeserializeOwned,
{
    get_global_config(context, key).await.unwrap_or(default)
}

pub async fn get_global_config<K, V>(context: &Context, key: K) -> Option<V>
where
    K: ToString,
    V: DeserializeOwned,
{
    let data = context.data.read().await;
    let container = data.get::<Zero2ConfigContainer>()?.read().await;

    container.global_config::<K, V>(key)
}

pub async fn get_guild_config_with_default<K, V>(
    context: &Context,
    guild_id: GuildId,
    key: K,
    default: V,
) -> V
where
    K: ToString,
    V: DeserializeOwned,
{
    get_guild_config(context, guild_id, key)
        .await
        .unwrap_or(default)
}

pub async fn get_guild_config<K, V>(context: &Context, guild_id: GuildId, key: K) -> Option<V>
where
    K: ToString,
    V: DeserializeOwned,
{
    let data = context.data.read().await;
    let container = data.get::<Zero2ConfigContainer>()?.read().await;

    container.guild_config::<K, V>(guild_id, key)
}
