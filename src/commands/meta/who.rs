use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::Context;
use serenity::utils::parse_username;

fn user_id_from_message(message: &Message, mut args: Args) -> Result<UserId, CommandError> {
    if args.is_empty() {
        return Ok(message.author.id);
    }

    let arg = args.single::<String>()?;

    if let Some(user_id) = parse_mention(&arg).map(UserId) {
        return Ok(user_id);
    }

    if let Ok(user_id) = arg.parse::<UserId>() {
        return Ok(user_id);
    }

    Err(CommandError::from("Error parsing mention"))
}

#[command]
async fn who(context: &Context, message: &Message, args: Args) -> CommandResult {
    let user_id = user_id_from_message(&message, args);

    let member: Member = match message.guild_id {
        Some(guild_id) => guild_id
            .member(&context, user_id)
            .await
            .map_err(|_| CommandError::from("The user is not a member of this guild."))?,
        None => return Err(CommandError::from("Unexpected Error! Seek help!")),
    };

    let colour = member
        .colour(&context)
        .await
        .unwrap_or_else(|| Colour::new(MAIN_COLOUR));

    let nick = member
        .nick
        .clone()
        .unwrap_or_else(|| member.display_name().to_string());

    let joined_date = match member.joined_at {
        Some(date) => date.format("%a, %B %e, %Y at %H:%M:%S").to_string(),
        None => "N/A".to_string(),
    };

    let mut roles = member
        .roles
        .into_iter()
        .map(|role_id| format!("<@&{}>", role_id))
        .collect::<Vec<String>>();

    roles.push("@everyone".to_string());

    let avatar_url = { member.user.read().face() };

    let _ = message
        .channel_id
        .send_message(&context, |m| {
            m.embed(|e| {
                e.title(nick)
                    .colour(colour)
                    .thumbnail(avatar_url)
                    .field("Roles", roles.join(" "), false)
                    .field("Joined", joined_date, false)
            })
        })
        .await;

    Ok(())
}
