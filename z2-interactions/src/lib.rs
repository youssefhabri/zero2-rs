mod anilist;
mod meta;
mod utils;

use serenity::model::interactions::Interaction;
use serenity::{
    model::prelude::{GuildId, InteractionData, InteractionResponseType},
    prelude::{Context, SerenityError},
};

pub async fn register_interactions(
    context: &Context,
    guild_id: GuildId,
) -> Result<(), SerenityError> {
    anilist::register_interactions(&context, guild_id)
        .await
        .unwrap();

    meta::register_interactions(&context, guild_id)
        .await
        .unwrap();

    Ok(())
}

pub async fn handle_interaction_create(context: &Context, interaction: Interaction) {
    let interaction_data = match interaction.data.as_ref() {
        Some(InteractionData::ApplicationCommand(data)) => data,
        _ => return,
    };

    match interaction_data.name.as_ref() {
        name if anilist::NAMES.contains(&name) => {
            let _resp = anilist::handle_interactions(&context, &interaction, name).await;
        }
        name if meta::NAMES.contains(&name) => {
            let _resp = meta::handle_interactions(&context, &interaction, name).await;
        }
        _ => {
            println!("Unhanlded interaction: {}", interaction.id);
        }
    }

    let _response = interaction
        .create_interaction_response(&context, |resp| resp.kind(InteractionResponseType::Pong))
        .await;
}
