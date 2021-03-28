mod anilist;
mod meta;
mod utils;

use serenity::model::interactions::Interaction;
use serenity::{
    builder::CreateInteractionResponse,
    model::prelude::{GuildId, InteractionResponseType},
    prelude::{Context, SerenityError},
};

pub async fn register_interactions(
    context: &Context,
    guild_id: GuildId,
) -> Result<(), SerenityError> {
    let application_info = context.http.get_current_application_info().await?;
    let application_id = application_info.id.as_u64();

    anilist::register_interactions(&context, guild_id, *application_id)
        .await
        .unwrap();

    meta::register_interactions(&context, guild_id, *application_id)
        .await
        .unwrap();

    Ok(())
}

pub async fn handle_interaction_create(context: &Context, interaction: Interaction) {
    let interaction_id = *interaction.id.as_u64();
    let interaction_token = interaction.token.as_str();

    if let Some(data) = interaction.data.as_ref() {
        match data.name.as_str() {
            name if anilist::NAMES.contains(&name) => {
                let _resp = anilist::handle_interactions(&context, &interaction, name).await;
            }
            name if meta::NAMES.contains(&name) => {
                let _resp = meta::handle_interactions(&context, &interaction, name).await;
            }
            _ => {
                println!("Unhanlded interaction: ID: {}", interaction_id);
            }
        }

        let resp = CreateInteractionResponse::default()
            .kind(InteractionResponseType::Acknowledge)
            .to_owned();

        let map = serde_json::Value::Object(serenity::utils::hashmap_to_json_map(resp.0));

        let _response = context
            .http
            .create_interaction_response(interaction_id, interaction_token, &map)
            .await;
    }
}
