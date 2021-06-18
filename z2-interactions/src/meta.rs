use serenity::model::prelude::{GuildId, Interaction, InteractionData, UserId};
use serenity::prelude::{Context, SerenityError};

use crate::utils::{regitser_command, CommandOption};

pub const NAMES: [&str; 1] = ["avatar"];

pub async fn register_interactions(
    context: &Context,
    guild_id: GuildId,
) -> Result<(), SerenityError> {
    let opts = vec![CommandOption::user(true)];
    let description = "Display's the user's avatar";
    regitser_command(&context, guild_id, "avatar", description, opts).await?;

    Ok(())
}

pub async fn handle_interactions(
    context: &Context,
    interaction: &Interaction,
    name: &str,
) -> Result<(), SerenityError> {
    match name {
        "avatar" => {
            let data = match interaction.data.as_ref() {
                Some(InteractionData::ApplicationCommand(data)) => data,
                _ => return Ok(()), // TODO display error to user
            };

            let user_id = data
                .options
                .iter()
                .find(|opt| opt.name == "user")
                .map(|opt| opt.value.clone())
                .flatten()
                .map(|val| val.as_str().map(|s| s.parse::<UserId>().ok()))
                .flatten()
                .flatten();

            let user = match user_id {
                Some(id) => match id.to_user(&context).await {
                    Ok(user) => user,
                    Err(_) => return Ok(()), // TODO Should be `Err()` not `Ok()`
                },
                None => return Ok(()), // TODO Should be `Err()` not `Ok()`
            };

            let channel_id = match interaction.channel_id {
                Some(id) => id,
                None => return Ok(()), // TODO Should be `Err()` not `Ok()`
            };

            let avatar_url = match user.avatar_url() {
                Some(avatar_url) => avatar_url,
                None => {
                    let _ = channel_id
                        .say(&context, format!("{} doesn't have an avatar.", &user.tag()))
                        .await;

                    return Ok(());
                }
            };

            let user_nick = user
                .nick_in(&context, interaction.guild_id.unwrap_or(GuildId(0)))
                .await
                .unwrap_or_else(|| user.name.clone());

            let _sent = channel_id
                .send_message(&context.http, |m| {
                    m.embed(|e| {
                        e.title(format!("{}'s avatar", user_nick))
                            .url(&avatar_url)
                            .image(avatar_url)
                    })
                })
                .await;
        }
        _ => {}
    }

    Ok(())
}
