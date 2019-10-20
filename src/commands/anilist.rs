use serenity::framework::standard::{macros::command, macros::group, Args, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::checks::*;
use crate::core::consts::{DB as db, OWNER_ID, PREFIX};
use crate::menu::{self, builders};
use crate::models::anilist::user::User;

pub mod client;
pub mod utils;

// command modules
pub mod activity;
pub mod airing;
pub mod anime;
pub mod character;
pub mod manga;
pub mod staff;
pub mod user;

use self::activity::ACTIVITY_COMMAND;
use self::airing::AIRING_COMMAND;
use self::anime::ANIME_COMMAND;
use self::character::CHARACTER_COMMAND;
use self::manga::MANGA_COMMAND;
use self::staff::STAFF_COMMAND;
use self::user::USER_COMMAND;

group!({
    name: "Anilist",
    commands: [anilist, activity, airing, anime, character, manga, staff, user]
});

#[command]
#[aliases(al)]
#[sub_commands(connect, stats)]
#[checks(user_exists, anilist_username)]
fn anilist(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    let user = db.find_user(message.author.id).unwrap();

    if user.anilist_name.is_empty() {
        let _ = message.channel_id.send_message(&context, |m| {
            m.content(format!(
                "You have not connected your anilist account yet. Please use \
                 `{}anilist connect [username]` to connect your anilist username.",
                PREFIX.as_str()
            ))
        });
        return Ok(());
    }

    let al_user = match client::search_user(user.anilist_name.clone()) {
        Some(user) => user,
        None => {
            let _ = message.channel_id.send_message(&context, |m| {
                m.content(format!(
                    "{} is not a valid AniList username. Please use a valid username \
                     (use `{}anilist connect [username]` to update your anilist username.)",
                    user.anilist_name,
                    PREFIX.as_str()
                ))
            });
            return Ok(());
        }
    };

    let _ = message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.clone_from(&builders::user_embed_builder(&al_user, "".to_string()));
            e
        })
    });

    Ok(())
}

#[command]
#[checks(user_exists)]
fn connect(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        let _ = message
            .channel_id
            .say(&context.http, "You need to input an anilist username");
        return Ok(());
    }

    let username = args.parse::<String>().unwrap_or_else(|_| "".to_string());
    // Get the user info from anilist
    let result = match client::search_user(username.clone()) {
        Some(user) => user,
        None => {
            let _ = message.channel_id.send_message(&context, |m| {
                m.content(format!("{} is not a valid anilist username", username))
            });

            return Ok(());
        }
    };

    let user_id = *message.author.id.as_u64() as i64;
    let message_content = match db.update_anilist_name(user_id, result.id as i32, result.name) {
        Ok(_user) => "You're anilist username has been added to your profile".to_string(),
        Err(why) => {
            warn!("Error while updating user({}) info: {}", user_id, why);
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
#[checks(user_exists, anilist_username)]
fn stats(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    let user = db.find_user(message.author.id).unwrap();
    let al_user = match client::search_user(user.anilist_name.clone()) {
        Some(user) => user,
        None => {
            let _ = message.channel_id.send_message(&context, |m| {
                m.content(format!(
                    "{} is not a valid AniList username. Please use a valid username \
                     (use {}anilist connect [username] to update your anilist username.)",
                    user.anilist_name,
                    PREFIX.as_str()
                ))
            });
            return Ok(());
        }
    };

    let response_msg = message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.clone_from(&builders::user_anime_stats_embed_builder(
                &al_user,
                "".to_string(),
            ));

            e
        })
        .reactions(menu::reactions::stats())
    });

    let pages = vec![
        builders::user_anime_stats_embed_builder(&al_user, "".to_string()),
        builders::user_manga_stats_embed_builder(&al_user, "".to_string()),
    ];

    if let Ok(response_msg) = response_msg {
        menu::new_pagination_with_handler(
            context,
            response_msg.id,
            message.author.id,
            pages,
            Some(menu::handlers::user_stats_handler),
        )
    }

    Ok(())
}
