use anilist::models::Media;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::{Message, Reaction, ReactionType};
use serenity::prelude::Context;

use super::AniListPagination;
use crate::anilist::embeds::{
    media_overview_embed, media_recommendations_embed, media_stats_embed,
};
use crate::anilist::types::AniListMediaView;
use crate::anilist::AniListPaginationKind;
use crate::types::Pagination;
use crate::{reactions, utils};

impl AniListPagination {
    pub async fn new_media_pagination(
        context: &Context,
        message: &Message,
        media: &[Media],
        view: AniListMediaView,
    ) -> CommandResult {
        let ids = media.iter().map(|media| media.id).collect();
        let kind = AniListPaginationKind::Media(view);
        let pagination = AniListPagination::new(ids, kind);

        let embed = pagination.media_embed(&media[0]);
        let reactions = reactions::media(media.len());
        let sent = utils::send_embed_message(&context, &message, &embed, reactions).await?;

        utils::add_pagination_to_store(&context, Box::new(pagination), sent.id, message.author.id)
            .await;

        Ok(())
    }

    pub(crate) fn set_media_view(&mut self, reaction: &Reaction) {
        self.kind = match reaction.emoji {
            ReactionType::Unicode(ref x) if x == reactions::OVERVIEW => {
                AniListPaginationKind::Media(AniListMediaView::Overview)
            }
            ReactionType::Unicode(ref x) if x == reactions::STATS => {
                AniListPaginationKind::Media(AniListMediaView::Stats)
            }
            ReactionType::Unicode(ref x) if x == reactions::RECOMMENDATIONS => {
                AniListPaginationKind::Media(AniListMediaView::Recommendations)
            }
            _ => return,
        };
    }

    fn media_footer(&self, media: &Media) -> String {
        // Page: 1/6 | Status: Finished | Powered by AniList
        let footer = format!(
            "Page: {}/{} | Status: {} | Powered by AniList",
            self.cursor() + 1,
            self.len(),
            media.status()
        );

        footer
    }

    pub fn media_embed(&self, media: &Media) -> CreateEmbed {
        match &self.kind {
            AniListPaginationKind::Media(view) => {
                let footer = Some(self.media_footer(&media));
                match view {
                    AniListMediaView::Overview => media_overview_embed(&media, footer),
                    AniListMediaView::Stats => media_stats_embed(&media, footer),
                    AniListMediaView::Recommendations => {
                        media_recommendations_embed(&media, footer)
                    }
                }
            }
            _ => CreateEmbed::default(),
        }
    }

    pub(crate) async fn _media_handler(&mut self, context: &Context, reaction: &Reaction) {
        let response = anilist::client::fetch_media(self.ids[self.cursor]).await;
        let media = match response {
            Ok(media) => media,
            Err(why) => {
                error!("MediaFetch error: {}", why);
                return;
            }
        };

        let embed = self.media_embed(&media);
        self.update_message(&context, &reaction, embed).await;
    }
}
