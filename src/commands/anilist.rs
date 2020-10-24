use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use anilist::models::MediaType;

use menu::anilist::{
    AniListCharacterView, AniListMediaView, AniListPagination, AniListStaffView, AniListUserView,
};

#[group]
#[commands(anime, manga, character, user, staff)]
struct AniList;

fn keyword_from_args(args: &mut Args) -> String {
    args.raw()
        .filter(|arg| !arg.starts_with('-'))
        .collect::<Vec<_>>()
        .join(" ")
}

async fn media(
    context: &Context,
    message: &Message,
    mut args: Args,
    media_type: MediaType,
) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from(format!(
            "No {} title was entered.",
            media_type.to_string()
        )));
    }

    let view = args.find::<AniListMediaView>().unwrap_or_default();
    let keyword = keyword_from_args(&mut args);
    let media = anilist::client::search_media(keyword, media_type).await?;

    AniListPagination::new_media_pagination(&context, &message, &media, view).await?;

    Ok(())
}

#[command]
#[aliases(a)]
async fn anime(context: &Context, message: &Message, args: Args) -> CommandResult {
    media(&context, &message, args, MediaType::Anime).await
}

#[command]
#[aliases(m)]
async fn manga(context: &Context, message: &Message, args: Args) -> CommandResult {
    media(&context, &message, args, MediaType::Manga).await
}

#[command]
#[aliases(u)]
async fn user(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("No username was entered."));
    }

    let keyword = keyword_from_args(&mut args);
    let users = anilist::client::search_user(keyword).await?;
    let view = args.find::<AniListUserView>().unwrap_or_default();

    AniListPagination::new_user_pagination(&context, &message, &users, view).await?;

    Ok(())
}

#[command]
#[aliases(c)]
async fn character(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("No character name was entered."));
    }

    let keyword = keyword_from_args(&mut args);
    let characters = anilist::client::search_character(keyword).await?;
    let view = args.find::<AniListCharacterView>().unwrap_or_default();

    AniListPagination::new_character_pagination(&context, &message, &characters, view).await?;

    Ok(())
}

#[command]
#[aliases(st)]
async fn staff(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("No staff name was entered."));
    }

    let view = args.find::<AniListStaffView>().unwrap_or_default();
    let keyword = keyword_from_args(&mut args);

    let staff = anilist::client::search_staff(keyword).await?;
    AniListPagination::new_staff_pagination(&context, &message, &staff, view).await?;

    Ok(())
}

#[command]
async fn activity(_context: &Context, _message: &Message, _args: Args) -> CommandResult {
    Ok(())
}
