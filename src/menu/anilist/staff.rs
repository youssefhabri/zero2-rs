use anilist::models::Staff;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::{Message, Reaction, ReactionType};
use serenity::prelude::Context;

use crate::core::store::Pagination;
use crate::menu::{reactions, utils};

use super::embeds::{staff_overview_embed, staff_related_anime_embed, staff_related_manga_embed};
use super::{AniListPagination, AniListPaginationKind, AniListStaffView};

impl AniListPagination {
    pub async fn new_staff_pagination(
        context: &Context,
        message: &Message,
        staff: &[Staff],
        view: AniListStaffView,
    ) -> CommandResult {
        let ids = staff.iter().map(|staff| staff.id).collect();
        let kind = AniListPaginationKind::Staff(view);
        let pagination = AniListPagination::new(ids, kind);

        let embed = pagination.staff_embed(&staff[0]);
        let reactions = reactions::staff(staff.len());
        let sent = utils::send_embed_message(&context, &message, &embed, reactions).await?;

        utils::add_pagination_to_store(&context, Box::new(pagination), sent.id, message.author.id)
            .await;

        Ok(())
    }

    pub(crate) fn set_staff_view(&mut self, reaction: &Reaction) {
        self.kind = match reaction.emoji {
            ReactionType::Unicode(ref x) if x == reactions::OVERVIEW => {
                AniListPaginationKind::Staff(AniListStaffView::Overview)
            }
            ReactionType::Unicode(ref x) if x == reactions::ANIME => {
                AniListPaginationKind::Staff(AniListStaffView::RelatedAnime)
            }
            ReactionType::Unicode(ref x) if x == reactions::MANGA => {
                AniListPaginationKind::Staff(AniListStaffView::RelatedManga)
            }
            _ => return,
        }
    }

    fn staff_footer(&self) -> String {
        let footer = "Powered by AniList".to_string();

        // Page: 1/6 | Powered by AniList
        if self.ids.len() > 1 {
            return format!("Page: {}/{} | {}", self.cursor() + 1, self.len(), footer);
        }

        footer
    }

    pub fn staff_embed(&self, staff: &Staff) -> CreateEmbed {
        match &self.kind {
            AniListPaginationKind::Staff(view) => {
                let footer = Some(self.staff_footer());
                match view {
                    AniListStaffView::Overview => staff_overview_embed(&staff, footer),
                    AniListStaffView::RelatedAnime => staff_related_anime_embed(&staff, footer),
                    AniListStaffView::RelatedManga => staff_related_manga_embed(&staff, footer),
                }
            }
            _ => CreateEmbed::default(),
        }
    }

    pub(crate) async fn _staff_handler(&mut self, context: &Context, reaction: &Reaction) {
        let response = anilist::client::fetch_staff(self.ids[self.cursor]).await;
        let user = match response {
            Ok(staff) => staff,
            Err(why) => {
                println!("StaffFetch error: {}", why);
                return;
            }
        };

        let embed = self.staff_embed(&user);
        self.update_message(&context, &reaction, embed).await;
    }
}
