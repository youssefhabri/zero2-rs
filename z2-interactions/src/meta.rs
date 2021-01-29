use serenity::model::prelude::{GuildId, Interaction, UserId};
use serenity::prelude::{Context, SerenityError};

use crate::utils::{regitser_command, CommandOption};

pub const NAMES: [&str; 1] = ["avatar"];

pub async fn register_interactions(
    context: &Context,
    guild_id: GuildId,
    app_id: u64,
) -> Result<(), SerenityError> {
    let opts = vec![CommandOption::user(true)];
    let description = "Display's the user's avatar";
    regitser_command(&context, guild_id, app_id, "avatar", description, opts).await?;

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
                Some(data) => data,
                None => return Ok(()), // TODO display error to user
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

            if let Some(user_id) = user_id {
                if let Ok(user) = user_id.to_user(&context).await {
                    let avatar_url = match user.avatar_url() {
                        Some(avatar_url) => avatar_url,
                        None => {
                            let _ = interaction
                                .channel_id
                                .say(&context, format!("{} doesn't have an avatar.", &user.tag()))
                                .await;

                            return Ok(());
                        }
                    };

                    let user_nick = user
                        .nick_in(&context, interaction.guild_id)
                        .await
                        .unwrap_or_else(|| user.name.clone());

                    let _sent = interaction
                        .channel_id
                        .send_message(&context.http, |m| {
                            m.embed(|e| {
                                e.title(format!("{}'s avatar", user_nick))
                                    .url(&avatar_url)
                                    .image(avatar_url)
                            })
                        })
                        .await;
                }
            }
        }
        _ => {}
    }

    Ok(())
}
