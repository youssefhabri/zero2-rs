use menu::anilist::{AniListPagination, AniListUserView};
use serenity::{
    model::prelude::{GuildId, Interaction, InteractionData},
    prelude::{Context, SerenityError},
};

use crate::utils::{regitser_command, CommandOption};

pub const NAMES: [&str; 3] = ["anime", "manga", "user"];

macro_rules! get_option {
    ($interaction:expr, $name:expr) => {
        match $interaction.data.as_ref() {
            Some(data) => match data {
                InteractionData::ApplicationCommand(cmd) => cmd
                    .options
                    .iter()
                    .find(|opt| opt.name == $name)
                    .map(|opt| opt.value.clone())
                    .flatten()
                    .map(|val| val.to_string()),
                _ => None,
            },
            None => None,
        }
        .ok_or(SerenityError::Other("Error getting Interaction Data"))?;
    };
}

pub async fn register_interactions(
    context: &Context,
    guild_id: GuildId,
) -> Result<(), SerenityError> {
    let opts = vec![CommandOption::string(
        "title",
        "Anime title to search for in AniList",
    )];
    let description = "Search for an anime in AniList";
    regitser_command(&context, guild_id, "anime", description, opts).await?;

    let opts = vec![CommandOption::string(
        "title",
        "Mange title to search for in AniList",
    )];
    let description = "Search for a manga in AniList";
    regitser_command(&context, guild_id, "manga", description, opts).await?;

    let opts = vec![CommandOption::string("name", "The user's username")];
    let description = "Search for a user in AniList";
    regitser_command(&context, guild_id, "user", description, opts).await?;

    Ok(())
}

pub async fn handle_interactions(
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

    let channel_id = match interaction.channel_id {
        Some(channel_id) => channel_id,
        None => {
            return Err(SerenityError::Other(
                "ChannelId is None. TODO: user custom error type",
            ))
        }
    };

    let media_type = anilist::models::MediaType::from(name);
    let media = anilist::client::search_media_with_adult(&title, media_type.clone(), false)
        .await
        .unwrap();

    if media.is_empty() {
        let content = format!("No {} was found for `{}`", media_type, title);
        channel_id.say(&context, content).await.unwrap();
        return Err(SerenityError::Other(
            "AniList Error. TODO: user custom error type",
        ));
    }

    let author_id = match interaction.user.as_ref() {
        Some(user) => user.id,
        None => {
            return Err(SerenityError::Other(
                "interaction.user is None. TODO: user custom error type",
            ))
        }
    };

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

    let channel_id = match interaction.channel_id {
        Some(channel_id) => channel_id,
        None => {
            return Err(SerenityError::Other(
                "ChannelId is None. TODO: user custom error type",
            ))
        }
    };

    let author_id = match interaction.user.as_ref() {
        Some(user) => user.id,
        None => {
            return Err(SerenityError::Other(
                "interaction.user is None. TODO: user custom error type",
            ))
        }
    };

    AniListPagination::new_user_pagination(
        &context,
        &channel_id,
        &author_id,
        &users,
        AniListUserView::Overview,
    )
    .await
    .map_err(|_err| SerenityError::Other("TODO"))?;

    Ok(())
}
