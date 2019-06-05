use serenity::framework::standard::{macros::check, Args, CheckResult, CommandOptions};
use serenity::model::channel::Message;
use serenity::prelude::*;

// A function which acts as a "check", to determine whether to call a command.
//
// This check analyses whether a guild member permissions has
// administrator-permissions.
#[check]
#[name = "Admin"]
// Whether the check shall be tested in the help-system.
#[check_in_help(true)]
// Whether the check shall be displayed in the help-system.
#[display_in_help(true)]
fn admin_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx.cache) {
        if let Ok(permissions) = member.permissions(&ctx.cache) {
            return permissions.administrator().into();
        }
    }

    false.into()
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

    false.into()
}
