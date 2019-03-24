use serenity::framework::standard::{Args, Command, CommandError};
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::utils::parse_mention;
use serenity::model::id::UserId;
use serenity::model::user::User;

pub struct Avatar;

impl Command for Avatar {
    fn execute(&self, _: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {

        if args.full().len() <= 0 {
            let _ = message.channel_id.say("You need to input a username.");
            return Ok(());
        }

        for user_tag in args.multiple::<String>().unwrap() {
            let user_id = parse_mention(user_tag.as_str());

            match user_id {
                Some(uid) => {
                    let user: User = UserId(uid).to_user().unwrap();

                    let _ = match user.avatar {
                        Some(_) => {

                            message.channel_id.send_message(|m| m
                                .embed(|e| e
                                    .title(format!("{}'s avatar",
                                                   user.nick_in(
                                                       message.guild_id.unwrap()
                                                   ).unwrap_or(user.name.clone())))
                                    .url(user.avatar_url().unwrap())
                                    .image(user.avatar_url().unwrap())
                                )
                            )
                        },
                        None => message.channel_id.say("The user doesn't have an avatar.")
                    };
                },
                None => {
                    let _ = message.channel_id.say(
                        format!("Something went wrong while fetch {}'s info.", user_tag)
                    );
                }
            };
        }

        Ok(())
    }
}