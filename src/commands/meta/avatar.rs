use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::{channel::Message, id::UserId, user::User};
use serenity::prelude::*;
use serenity::utils::parse_mention;

use std::result::Result;

#[command]
fn avatar(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let user_tags = if args.is_empty() {
        vec![format!("<@!{}>", message.author.id)]
    } else {
        args.iter::<String>()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect::<Vec<_>>()
    };

    for user_tag in user_tags {
        let user_id = parse_mention(user_tag.as_str());

        match user_id {
            Some(uid) => {
                let user: User = UserId(uid).to_user(&context).unwrap();

                let _ = match user.avatar {
                    Some(_) => message.channel_id.send_message(&context.http, |m| {
                        m.embed(|e| {
                            e.title(format!(
                                "{}'s avatar",
                                user.nick_in(&context, message.guild_id.unwrap())
                                    .unwrap_or_else(|| user.name.clone())
                            ))
                            .url(user.avatar_url().unwrap())
                            .image(user.avatar_url().unwrap())
                        })
                    }),
                    None => message
                        .channel_id
                        .say(&context.http, "The user doesn't have an avatar."),
                };
            }
            None => {
                let _ = message.channel_id.say(
                    &context.http,
                    format!("Something went wrong while fetching {}'s info.", user_tag),
                );
            }
        };
    }

    Ok(())
}
