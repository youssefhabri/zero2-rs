#![feature(or_patterns)]

mod anilist;
mod utils;

use serenity::{
    builder::CreateInteractionResponse,
    model::interactions::InteractionResponseType,
    prelude::{Context, SerenityError},
};
use serenity::{http::Http, model::interactions::Interaction};
use std::sync::Arc;

pub async fn register_interactions(http: Arc<Http>) -> Result<(), SerenityError> {
    let application_info = http.get_current_application_info().await?;
    let application_id = application_info.id.as_u64();

    anilist::register_anilist_interactions(http, *application_id)
        .await
        .unwrap();

    Ok(())
}

pub async fn handle_interaction_create(context: &Context, interaction: Interaction) {
    let interaction_id = *interaction.id.as_u64();
    let interaction_token = interaction.token.as_str();

    if let Some(data) = interaction.data.as_ref() {
        match data.name.as_str() {
            name @ ("anime" | "manga" | "user") => {
                let _resp =
                    anilist::handle_anilist_interactions(&context, &interaction, name).await;
            }
            _ => {}
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
