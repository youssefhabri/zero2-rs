use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::{Member, Message, UserId};
use serenity::prelude::Context;
use serenity::utils::{parse_mention, Colour};

use crate::core::consts::MAIN_COLOUR;

#[command]
fn who(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let user_id: UserId = user_id_from_message(message, args)?;

    let member: Member = match message.guild_id {
        Some(guild_id) => guild_id.member(&context, user_id)?,
        None => return Err(CommandError::from("Unexpected Error! Seek help!")),
    };

    let colour = member.colour(&context).unwrap_or(Colour::new(MAIN_COLOUR));

    let nick = member
        .nick
        .clone()
        .unwrap_or(member.display_name().to_string());

    let joined_date = match member.joined_at {
        Some(date) => date.format("%a, %B %e, %Y at %H:%M:%S").to_string(),
        None => "N/A".to_string(),
    };

    let roles = {
        member
            .roles
            .into_iter()
            .map(|role_id| format!("<@&{}>", role_id))
            .collect::<Vec<String>>()
            .join(" ")
    };

    let avatar_url = { member.user.read().face() };

    let _ = message.channel_id.send_message(&context, |m| {
        m.embed(|e| {
            e.title(nick)
                .colour(colour)
                .thumbnail(avatar_url)
                .field("Roles", roles, false)
                .field("Joined", joined_date, false)
        })
    });

    Ok(())
}

fn user_id_from_message(message: &Message, mut args: Args) -> Result<UserId, CommandError> {
    if args.is_empty() {
        return Ok(message.author.id);
    }

    let arg = args.single::<String>()?;

    if let Some(user_id) = parse_mention(arg.clone()).map(UserId) {
        return Ok(user_id);
    }

    if let Ok(user_id) = arg.parse::<UserId>() {
        return Ok(user_id);
    }

    Err(CommandError::from("Error parsing mention"))
}
