use anilist::models::Staff;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::{Message, Reaction, ReactionType};
use serenity::prelude::Context;

use crate::anilist::embeds::{
    staff_overview_embed, staff_related_anime_embed, staff_related_manga_embed,
};
use crate::anilist::{AniListPagination, AniListPaginationKind, AniListStaffView};
use crate::types::PaginationResult;
use crate::{reactions, utils};

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
        let sent =
            utils::send_embed_message(&context, &message.channel_id, &embed, reactions).await?;

        utils::add_pagination_to_store(&context, pagination, sent.id, message.author.id).await;

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

    pub fn staff_embed(&self, staff: &Staff) -> CreateEmbed {
        match &self.kind {
            AniListPaginationKind::Staff(view) => {
                let footer = Some(self.standard_footer());
                match view {
                    AniListStaffView::Overview => staff_overview_embed(&staff, footer),
                    AniListStaffView::RelatedAnime => staff_related_anime_embed(&staff, footer),
                    AniListStaffView::RelatedManga => staff_related_manga_embed(&staff, footer),
                }
            }
            _ => CreateEmbed::default(),
        }
    }

    pub(crate) async fn _staff_handler(
        &mut self,
        context: &Context,
        reaction: &Reaction,
    ) -> PaginationResult {
        let staff = anilist::client::fetch_staff(self.ids[self.cursor]).await?;
        let embed = self.staff_embed(&staff);
        self.update_message(&context, &reaction, embed).await;

        Ok(())
    }
}
