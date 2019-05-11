use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::utils::parse_mention;
use serenity::model::id::UserId;
use serenity::model::user::User;


#[command("avatar")]
fn avatar_command(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {

    if args.parse::<String>().unwrap_or_else(|_| "".to_string()).is_empty() {
        let _ = message.channel_id.say(&context.http, "You need to input a username.");
        return Ok(());
    }

    for user_tag in args.iter::<String>() {
        let user_id = parse_mention(
            user_tag.unwrap_or_else(|_| "".to_string()).as_str());

        match user_id {
            Some(uid) => {
                let user: User = UserId(uid).to_user(&context).unwrap();

                let _ = match user.avatar {
                    Some(_) => {

                        message.channel_id.send_message(
                            &context.http,
                            |m| m
                                .embed(|e| e
                                    .title(format!("{}'s avatar",
                                                   user.nick_in(
                                                       &context,
                                                       message.guild_id.unwrap()
                                                   ).unwrap_or_else(|| user.name.clone())))
                                    .url(user.avatar_url().unwrap())
                                    .image(user.avatar_url().unwrap())
                                )
                        )
                    },
                    None => message.channel_id.say(&context.http, "The user doesn't have an avatar.")
                };
            },
            None => {
                let _ = message.channel_id.say(
                    &context.http,
                    format!(
                        // TODO Find a solution to this issue
                        // "Something went wrong while fetching {}'s info.", user_tag.unwrap()
                        "Something went wrong while fetching the user's info."
                    )
                );
            }
        };
    }

    Ok(())
}
