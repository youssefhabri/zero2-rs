use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::Context;
use serenity::utils::parse_username;

fn parse_id(s: String) -> Option<UserId> {
    let id = if s.starts_with('<') {
        parse_username(s)?
    } else {
        s.parse().ok()?
    };

    Some(UserId(id))
}

#[command]
async fn avatar(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let mut user_ids: Vec<UserId> = args
        .iter::<String>()
        .filter_map(Result::ok)
        .filter_map(parse_id)
        .collect();

    if user_ids.is_empty() {
        user_ids.push(message.author.id);
    };

    for id in user_ids {
        let user = match id.to_user(&context).await {
            Ok(user) => user,
            Err(_) => continue,
        };

        let avatar_url = user.face();

        let user_nick = user
            .nick_in(&context, message.guild_id.unwrap())
            .await
            .unwrap_or_else(|| format!("{}#{}", user.name, user.discriminator));

        let _sent = message
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

    Ok(())
}
