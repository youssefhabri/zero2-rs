use async_trait::async_trait;
use serenity::model::prelude::Reaction;
use serenity::{builder::CreateEmbed, prelude::Context};

use anilist::models::AniListID;

use crate::anilist::types::AniListPaginationKind;
use crate::types::{Pagination, PaginationResult};

pub struct AniListPagination {
    pub(crate) ids: Vec<AniListID>,
    pub(crate) kind: AniListPaginationKind,
    pub(crate) prev_kind: AniListPaginationKind,
    pub(crate) cursor: usize,
    pub(crate) prev_cursor: usize,
}

#[async_trait]
impl Pagination for AniListPagination {
    async fn handle(&mut self, context: &Context, reaction: &Reaction) -> PaginationResult {
        // Stop if neither the view nor the page has changed
        if !self.should_update(reaction).await {
            return Ok(());
        }

        match self.kind {
            AniListPaginationKind::AiringSchedule(_) => {
                self._airing_schedule_handler(&context, &reaction).await
            }
            AniListPaginationKind::Character(_) => {
                self._character_handler(&context, &reaction).await
            }
            AniListPaginationKind::Media(_) => self._media_handler(&context, &reaction).await,
            AniListPaginationKind::User(_) => self._user_handler(&context, &reaction).await,
            AniListPaginationKind::Staff(_) => self._staff_handler(&context, &reaction).await,
            AniListPaginationKind::Studio => self._studio_handler(&context, &reaction).await,
        }
    }

    fn len(&self) -> usize {
        self.ids.len()
    }

    fn cursor(&self) -> usize {
        self.cursor
    }

    fn set_cursor(&mut self, cursor: usize) {
        self.prev_cursor = self.cursor;
        self.cursor = cursor;
    }
}

impl AniListPagination {
    pub fn new(ids: Vec<AniListID>, kind: AniListPaginationKind) -> AniListPagination {
        AniListPagination {
            ids,
            kind: kind.clone(),
            prev_kind: kind,
            cursor: 0,
            prev_cursor: 0,
        }
    }

    pub(crate) async fn update_message(
        &self,
        context: &Context,
        reaction: &Reaction,
        embed: CreateEmbed,
    ) {
        let sent = reaction
            .channel_id
            .edit_message(&context, reaction.message_id, |m| {
                m.embed(|e| {
                    e.clone_from(&embed);
                    e
                })
            })
            .await;

        if let Err(why) = sent {
            error!("UpdateMessage Error: {}", why);
        }
    }

    pub(crate) fn standard_footer(&self) -> String {
        let footer = "Powered by AniList".to_string();

        // Page: 1/6 | Powered by AniList
        if self.ids.len() > 1 {
            return format!("Page: {}/{} | {}", self.cursor() + 1, self.len(), footer);
        }

        footer
    }

    async fn should_update(&mut self, reaction: &Reaction) -> bool {
        let prev_kind = self.kind.clone();
        match self.kind {
            AniListPaginationKind::AiringSchedule(_) => {
                self.set_airing_schedule_view(reaction).await
            }
            AniListPaginationKind::Character(_) => self.set_character_view(reaction),
            AniListPaginationKind::Media(_) => self.set_media_view(reaction),
            AniListPaginationKind::User(_) => self.set_user_view(reaction),
            AniListPaginationKind::Staff(_) => self.set_staff_view(reaction),
            AniListPaginationKind::Studio => {}
        }

        self.kind != prev_kind || self.cursor() != self.prev_cursor
    }
}
