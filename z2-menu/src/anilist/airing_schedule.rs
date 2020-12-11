use crate::anilist::embeds::{airing_schedule_embed, airing_schedule_main_embed};
use crate::anilist::types::ALAiringScheduleView;
use crate::anilist::{AniListPagination, AniListPaginationKind};
use crate::types::{Pagination, PaginationResult};
use crate::{reactions, utils};
use anilist::models::AiringSchedule;
use chrono::Weekday;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandResult;
use serenity::futures::executor::block_on;
use serenity::model::prelude::{Message, Reaction, ReactionType};
use serenity::prelude::Context;

impl AniListPagination {
    pub async fn new_airing_schedule_pagination(
        context: &Context,
        message: &Message,
        weekday: Option<Weekday>,
    ) -> CommandResult {
        let mut pagination = AniListPagination::new(
            Vec::new(),
            AniListPaginationKind::AiringSchedule(ALAiringScheduleView::from(weekday)),
        );

        let airing_schedule = if let Some(weekday) = weekday {
            block_on(async {
                pagination._update_airing_schedule_ids(weekday).await;
                anilist::client::fetch_airing_schedule_with_media_id(pagination.ids[0])
                    .await
                    .ok()
            })
        } else {
            None
        };

        let embed = pagination.airing_schedule_embed(airing_schedule);
        let reactions = reactions::airing_schedule_from_weekday(weekday, pagination.ids.len());

        let sent = utils::send_embed_message(&context, &message, &embed, reactions).await?;

        utils::add_pagination_to_store(&context, pagination, sent.id, message.author.id).await;

        Ok(())
    }

    async fn _update_airing_schedule_ids(&mut self, weekday: Weekday) {
        let date = utils::weekday_to_date(weekday);
        let start_date = date.and_hms(0, 0, 0).timestamp() as u64;
        let end_date = date.and_hms(23, 59, 59).timestamp() as u64;

        match anilist::client::fetch_airing_schedule_list(start_date, end_date).await {
            Ok(airing_schedule) => {
                self.ids = airing_schedule
                    .iter()
                    .map(|air_sch| air_sch.media.id)
                    .collect();

                self.cursor = 0;
                self.prev_cursor = 0;
            }
            Err(why) => error!("{}", why),
        }
    }

    pub(crate) async fn set_airing_schedule_view(&mut self, reaction: &Reaction) {
        self.prev_kind = self.kind.clone();
        self.kind = match reaction.emoji {
            ReactionType::Unicode(ref x) if x == reactions::HOME => {
                AniListPaginationKind::AiringSchedule(ALAiringScheduleView::Main)
            }

            ReactionType::Unicode(ref x) => {
                if let Some(weekday) = utils::reaction_to_weekday(x.as_str()) {
                    self._update_airing_schedule_ids(weekday).await;
                }
                AniListPaginationKind::AiringSchedule(ALAiringScheduleView::Schedule)
            }
            _ => return,
        };
    }

    fn airing_schedule_footer(&self) -> String {
        let footer = "Powered by AniList".to_string();

        // Page: 1/6 | Powered by AniList
        if self.ids.len() > 1 {
            return format!("Page: {}/{} | {}", self.cursor() + 1, self.len(), footer);
        }

        footer
    }

    fn airing_schedule_embed(&self, air_schedule: Option<AiringSchedule>) -> CreateEmbed {
        match &self.kind {
            AniListPaginationKind::AiringSchedule(view) => {
                let footer = Some(self.airing_schedule_footer());
                match view {
                    ALAiringScheduleView::Main => airing_schedule_main_embed(footer),
                    ALAiringScheduleView::Schedule => {
                        airing_schedule_embed(&air_schedule.unwrap(), footer)
                    }
                }
            }
            _ => CreateEmbed::default(),
        }
    }

    async fn set_new_reaction(&self, context: &Context, reaction: &Reaction) {
        if self.kind == self.prev_kind {
            return;
        }

        let new_reactions = match self.kind {
            AniListPaginationKind::AiringSchedule(ALAiringScheduleView::Schedule) => {
                reactions::airing_schedule_media(self.ids.len())
            }
            _ => reactions::airing_schedule_main(),
        };

        let message: Message = reaction.message(&context).await.unwrap();
        let _ = message.delete_reactions(&context).await;

        for reaction in new_reactions {
            let _ = message.react(&context, reaction).await;
        }
    }

    pub(crate) async fn _airing_schedule_handler(
        &mut self,
        context: &Context,
        reaction: &Reaction,
    ) -> PaginationResult {
        let airing_schedule = match self.kind.airing_schedule_view() {
            Some(ALAiringScheduleView::Schedule) => {
                let id = self.ids[self.cursor];
                anilist::client::fetch_airing_schedule_with_media_id(id)
                    .await
                    .ok()
            }
            _ => None,
        };

        let embed = self.airing_schedule_embed(airing_schedule);
        self.update_message(&context, &reaction, embed).await;
        self.set_new_reaction(&context, &reaction).await;

        Ok(())
    }
}
