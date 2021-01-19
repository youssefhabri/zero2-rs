use serenity::{
    builder::{CreateInteraction, CreateInteractionOption},
    http::Http,
    model::interactions::ApplicationCommandOptionType,
    prelude::SerenityError,
};
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref GUILD_ID: u64 = kankyo::key("INTERACTIONS_GUILD_ID").unwrap().parse().unwrap();
}

pub async fn regitser_command(
    http: &Arc<Http>,
    app_id: u64,
    name: &str,
    description: &str,
    opts: Vec<(&str, &str, bool)>,
) -> Result<(), SerenityError> {
    let options = opts
        .into_iter()
        .map(|(name, description, required)| {
            CreateInteractionOption::default()
                .name(name)
                .description(description)
                .required(required)
                .kind(ApplicationCommandOptionType::String)
                .to_owned()
        })
        .collect();

    let anime_command = CreateInteraction::default()
        .name(name)
        .description(description)
        .set_interaction_options(options)
        .to_owned();

    let map = serenity::utils::hashmap_to_json_map(anime_command.0);
    let map = serde_json::Value::Object(map);

    http.create_guild_application_command(app_id, GUILD_ID.to_owned(), &map)
        .await?;

    Ok(())
}
