use serenity::framework::standard::{macros::check, Args, CheckResult, CommandOptions};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::core::consts::{DB as db, PREFIX};

// A function which acts as a "check", to determine whether to call a command.
//
// This check analyses whether a guild member permissions has
// administrator-permissions.
#[check]
#[name = "Admin"]
#[check_in_help(true)]
#[display_in_help(true)]
fn admin_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx.cache) {
        if let Ok(permissions) = member.permissions(&ctx.cache) {
            return permissions.administrator().into();
        }
    }

    CheckResult::new_user("User is not an admin")
}

#[check]
#[name = "EAP"]
fn eap_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx.cache) {
        if let Some(roles) = member.roles(&ctx.cache) {
            return roles
                .iter()
                .any(|role| {
                    role.name == "Nitro Booster"
                        || role.name == "Donator"
                        || role.name == "Early Access"
                })
                .into();
        }
    }

    CheckResult::new_user("User does not have the Early Access, Donator, or the Nitro Booster role")
}

#[check]
#[name = "user_exists"]
fn user_exists_check(
    context: &mut Context,
    message: &Message,
    _args: &mut Args,
    _: &CommandOptions,
) -> CheckResult {
    match db.find_user(message.author.id) {
        Ok(_user) => CheckResult::Success,
        Err(why) => {
            let _ = message.channel_id.send_message(&context, |m| m.content(
                format!("You account is not registered in the database. Please run `{}profile init` to initialize it.", PREFIX.as_str())
            ));

            dbg!(why);

            CheckResult::new_user("User does not exists in the database")
        }
    }
}

#[check]
#[name = "anilist_username"]
fn anilist_username_check(
    context: &mut Context,
    message: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> CheckResult {
    let user = db.find_user(message.author.id).unwrap();
    if user.anilist_name.is_empty() {
        let _ = message.channel_id.send_message(&context, |m| {
            m.content(format!(
                "You have not connected your anilist account yet. Please use \
                 `{}anilist connect [username]` to connect your anilist username.",
                PREFIX.as_str()
            ))
        });
        return CheckResult::new_user("User hasn't connect their anilist account");
    }

    CheckResult::Success
}
