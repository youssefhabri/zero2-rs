use serenity::framework::standard::{macros::command, macros::group, Args, CommandResult};
use serenity::model::{channel::Message, id::GuildId, id::UserId};
use serenity::prelude::Context;
use serenity::utils::parse_username;

use crate::checks::*;
use crate::core::consts::{DB as db, OWNER_ID};

group!({
    name: "Profile",
    options: {
        prefixes: ["profile"],
        default_command: info,
        owner_privilege: false
    },
    commands: [info, init, delete]
});

#[command]
#[checks(user_exists)]
fn info(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    match db.find_user(message.author.id) {
        Ok(user) => {
            let _ = message.channel_id.send_message(&context, |m| {
                m.embed(|e| {
                    e.field("UserID", UserId(user.id as u64), true)
                        .field("GuildID", GuildId(user.guild_id as u64), true)
                        .field("AnilistID", user.anilist_id.unwrap_or(0), true)
                        .field("AnilistName", user.anilist_name, true)
                })
            });
        }
        Err(why) => {
            let _ = message
                .channel_id
                .send_message(&context, |m| m.content(why));
        }
    }
    Ok(())
}

#[command]
fn init(context: &mut Context, message: &Message) -> CommandResult {
    if db.find_user(message.author.id).is_ok() {
        let _ = message.channel_id.send_message(&context, |m| {
            m.content("Your account has already been created!".to_string())
        });

        return Ok(());
    };

    let _sent = message.channel_id.send_message(&context, |m| {
        m.content(format!(
            "Initializing <@{}>'s profile ...",
            message.author.id
        ))
    });

    // TODO handle the user profile already existing
    //  check if the user exists in the guild
    let roles = message
        .guild_id
        .unwrap()
        .member(&context, message.author.id)
        .unwrap()
        .roles;
    let username = message
        .author
        .nick_in(&context, message.guild_id.unwrap())
        .unwrap_or_else(|| message.author.name.clone());

    let message_content = match db.new_user(
        message.author.id,
        message.guild_id.unwrap_or(GuildId(0)),
        username,
        roles,
    ) {
        Ok(_user) => "You're AniTrend profile has been initialized".to_string(),
        Err(why) => {
            warn!(
                "Error while creating new user({}): {}",
                message.author.id, why
            );
            format!(
                "Something went wrong! Pray that <@{}> will do something about it!",
                OWNER_ID.as_str()
            )
        }
    };

    let _ = message
        .channel_id
        .send_message(&context, |m| m.content(message_content));

    Ok(())
}

#[command]
#[owners_only]
fn delete(_context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        return Ok(());
    }

    let user_ids: Vec<u64> = args
        .iter::<String>()
        .filter(Result::is_ok)
        .map(|tag| parse_username(tag.unwrap()))
        .filter(|parsed_tag| parsed_tag.is_some())
        .map(|user_id| user_id.unwrap())
        .collect();

    for user_id in user_ids {
        dbg!(db.delete_user(UserId(user_id), message.guild_id.unwrap()))?;
    }

    Ok(())
}
