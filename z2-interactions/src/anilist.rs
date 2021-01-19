use menu::anilist::{AniListPagination, AniListUserView};
use serenity::{
    http::Http,
    model::interactions::Interaction,
    prelude::{Context, SerenityError},
};
use std::sync::Arc;

use crate::utils::regitser_command;

macro_rules! get_option {
    ($interaction:expr, $name:expr) => {
        $interaction
            .data
            .as_ref()
            .map(|data| {
                data.options
                    .iter()
                    .find(|opt| opt.name == $name)
                    .map(|opt| opt.value.clone())
                    .flatten()
                    .map(|val| val.to_string())
            })
            .flatten()
            .ok_or(SerenityError::Other("Error getting Interaction Data"))?;
    };
}

pub async fn register_anilist_interactions(
    http: Arc<Http>,
    app_id: u64,
) -> Result<(), SerenityError> {
    let opts = vec![("title", "Anime title to search for in AniList", true)];
    let description = "Search for an anime in AniList";
    regitser_command(&http, app_id, "anime", description, opts).await?;

    let opts = vec![("title", "Mange title to search for in AniList", true)];
    let description = "Search for a manga in AniList";
    regitser_command(&http, app_id, "manga", description, opts).await?;

    let opts = vec![("name", "The user's username", true)];
    let description = "Search for a user in AniList";
    regitser_command(&http, app_id, "user", description, opts).await?;

    Ok(())
}

pub async fn handle_anilist_interactions(
    context: &Context,
    interaction: &Interaction,
    name: &str,
) -> Result<(), SerenityError> {
    match name {
        name @ ("anime" | "manga") => {
            handle_media_interaction(&context, &interaction, name).await?
        }
        "user" => handle_user_interaction(&context, &interaction).await?,
        _ => {}
    }

    Ok(())
}

async fn handle_media_interaction(
    context: &Context,
    interaction: &Interaction,
    name: &str,
) -> Result<(), SerenityError> {
    let title = get_option!(interaction, "title");

    let media_type = anilist::models::MediaType::from(name);

    let media = anilist::client::search_media_with_adult(&title, media_type.clone(), false)
        .await
        .unwrap();

    if media.is_empty() {
        let content = format!("No {} was found for `{}`", media_type, title);
        interaction.channel_id.say(&context, content).await.unwrap();
        return Err(SerenityError::Other(
            "AniList Error. TODO: user custom error type",
        ));
    }

    let author_id = interaction.member.user.id;
    let channel_id = interaction.channel_id;

    menu::anilist::AniListPagination::new_media_pagination(
        context,
        &channel_id,
        &author_id,
        &media,
        menu::anilist::AniListMediaView::Overview,
    )
    .await
    .unwrap();

    Ok(())
}

async fn handle_user_interaction(
    context: &Context,
    interaction: &Interaction,
) -> Result<(), SerenityError> {
    let username = get_option!(interaction, "name");
    let users = anilist::client::search_user(username)
        .await
        .map_err(|_err| SerenityError::Other("TODO"))?;

    if users.is_empty() {
        return Err(SerenityError::Other("TODO"));
    }

    AniListPagination::new_user_pagination(
        &context,
        &interaction.channel_id,
        &interaction.member.user.id,
        &users,
        AniListUserView::Overview,
    )
    .await
    .map_err(|_err| SerenityError::Other("TODO"))?;

    Ok(())
}
