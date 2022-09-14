mod anilist;
mod meta;
mod utils;

use serenity::{
    model::application::interaction::{Interaction, InteractionResponseType},
    model::prelude::GuildId,
    prelude::{Context, SerenityError},
};

pub async fn register_interactions(
    context: &Context,
    guild_id: GuildId,
) -> Result<(), SerenityError> {
    let guild_name = match guild_id.to_guild_cached(&context) {
        Some(guild) => guild.name,
        None => guild_id.to_string(),
    };

    if let Err(why) = anilist::register_interactions(&context, guild_id).await {
        println!(
            "[ERROR] Failed to register AniList interactions in {} : {}",
            guild_name, why
        );
    }

    if let Err(why) = meta::register_interactions(&context, guild_id).await {
        println!(
            "[ERROR] Failed to register Meta interactions in {} : {}",
            guild_name, why
        );
    }

    Ok(())
}

pub async fn handle_interaction_create(context: &Context, interaction: Interaction) {
    let application_command = match utils::get_application_command(&interaction) {
        Ok(application_command) => application_command,
        _ => return,
    };
    let interaction_data = &application_command.data;

    match interaction_data.name.as_ref() {
        name if anilist::NAMES.contains(&name) => {
            let _resp = anilist::handle_interactions(&context, &interaction, name).await;
        }
        name if meta::NAMES.contains(&name) => {
            let _resp = meta::handle_interactions(&context, &interaction, name).await;
        }
        _ => {
            println!("Unhanlded interaction: {}", application_command.id);
        }
    }

    let _response = application_command
        .create_interaction_response(&context, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
        })
        .await;
}
