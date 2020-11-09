use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::{
    core::config::Zero2ConfigContainer,
    utils::{get_global_config, get_guild_config},
};

#[group]
#[prefix(config)]
#[commands(get, set)]
#[default_command(get)]
struct Configuration;

#[command]
async fn get(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("No config name was entered."));
    }

    let first_part = args.parse::<String>()?.to_lowercase();
    dbg!(&first_part);
    let config_value = match first_part.as_str() {
        "global" => {
            let config_name = args
                .advance()
                .remains()
                .ok_or_else(|| CommandError::from("Error parsing remaining args"))?;
            dbg!(&config_name);
            get_global_config::<_, String>(&context, config_name).await
        }
        _ => {
            let config_name = args.message();
            match message.guild_id {
                Some(guild_id) => {
                    get_guild_config::<_, String>(&context, guild_id, config_name).await
                }
                None => None,
            }
        }
    };

    let _ = message
        .channel_id
        .say(&context, format!("{:?}", config_value))
        .await;

    Ok(())
}

#[command]
async fn set(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let config_name = args.parse::<String>()?;
    let config_value = args
        .advance()
        .remains()
        .ok_or_else(|| CommandError::from("Error parsing remaining args"))?;

    let mut data = context.data.write().await;
    let mut container = data
        .get_mut::<Zero2ConfigContainer>()
        .ok_or_else(|| CommandError::from("Error getting Zero2ConfigContainer"))?
        .write()
        .await;

    let result = container.set_global_config(config_name, config_value);

    let _ = message
        .channel_id
        .say(&context, format!("{:?}", result))
        .await;

    Ok(())
}
