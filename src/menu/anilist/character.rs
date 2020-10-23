use anilist::models::Character;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::{Message, Reaction, ReactionType};
use serenity::prelude::Context;

use super::embeds::character_overview_embed;
use super::{AniListCharacterView, AniListPagination, AniListPaginationKind};
use crate::core::store::Pagination;
use crate::menu::anilist::embeds::{character_related_anime_embed, character_related_manga_embed};
use crate::menu::{reactions, utils};

impl AniListPagination {
    pub async fn new_character_pagination(
        context: &Context,
        message: &Message,
        characters: &[Character],
        view: AniListCharacterView,
    ) -> CommandResult {
        let ids = characters.iter().map(|character| character.id).collect();
        let kind = AniListPaginationKind::Character(view);
        let pagination = AniListPagination::new(ids, kind);
        let embed = pagination.character_embed(&characters[0]);
        let reactions = reactions::character(characters.len());

        let sent = utils::send_embed_message(&context, &message, &embed, reactions).await?;

        utils::add_pagination_to_store(&context, Box::new(pagination), sent.id, message.author.id)
            .await;

        Ok(())
    }

    pub(crate) fn set_character_view(&mut self, reaction: &Reaction) {
        self.kind = match reaction.emoji {
            ReactionType::Unicode(ref x) if x == reactions::OVERVIEW => {
                AniListPaginationKind::Character(AniListCharacterView::Overview)
            }
            ReactionType::Unicode(ref x) if x == reactions::ANIME => {
                AniListPaginationKind::Character(AniListCharacterView::RelatedAnime)
            }
            ReactionType::Unicode(ref x) if x == reactions::MANGA => {
                AniListPaginationKind::Character(AniListCharacterView::RelatedManga)
            }
            _ => return,
        };
    }

    fn character_footer(&self) -> String {
        let footer = "Powered by AniList".to_string();

        // Page: 1/6 | Powered by AniList
        if self.ids.len() > 1 {
            return format!("Page: {}/{} | {}", self.cursor() + 1, self.len(), footer);
        }

        footer
    }

    pub(crate) fn character_embed(&self, character: &Character) -> CreateEmbed {
        match &self.kind {
            AniListPaginationKind::Character(view) => {
                let footer = Some(self.character_footer());
                match view {
                    AniListCharacterView::Overview => character_overview_embed(&character, footer),
                    AniListCharacterView::RelatedAnime => {
                        character_related_anime_embed(&character, footer)
                    }
                    AniListCharacterView::RelatedManga => {
                        character_related_manga_embed(&character, footer)
                    }
                }
            }
            _ => CreateEmbed::default(),
        }
    }

    pub(crate) async fn _character_handler(&mut self, context: &Context, reaction: &Reaction) {
        let response = anilist::client::fetch_character(self.ids[self.cursor]).await;
        let character = match response {
            Ok(user) => user,
            Err(why) => {
                println!("UserFetch error: {}", why);
                return;
            }
        };

        let embed = self.character_embed(&character);
        self.update_message(&context, &reaction, embed).await;
    }
}
