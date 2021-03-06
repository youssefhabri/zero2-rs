use anilist::models::User;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::{ChannelId, Reaction, ReactionType, UserId};
use serenity::prelude::Context;

use crate::anilist::embeds::{user_favourites_embed, user_overview_embed, user_stats_embed};
use crate::anilist::types::{AniListPaginationKind, AniListUserView};
use crate::anilist::AniListPagination;
use crate::types::PaginationResult;
use crate::{reactions, utils};

impl AniListPagination {
    pub async fn new_user_pagination(
        context: &Context,
        channel_id: &ChannelId,
        author_id: &UserId,
        users: &[User],
        view: AniListUserView,
    ) -> CommandResult {
        let ids = users.iter().map(|user| user.id).collect();
        let kind = AniListPaginationKind::User(view);
        let pagination = AniListPagination::new(ids, kind);

        let embed = pagination.user_embed(&users[0]);
        let reactions = reactions::user(users.len());
        let sent = utils::send_embed_message(&context, &channel_id, &embed, reactions).await?;

        utils::add_pagination_to_store(&context, pagination, sent.id, *author_id).await;

        Ok(())
    }

    pub(crate) fn set_user_view(&mut self, reaction: &Reaction) {
        self.kind = match reaction.emoji {
            ReactionType::Unicode(ref x) if x == reactions::OVERVIEW => {
                AniListPaginationKind::User(AniListUserView::Overview)
            }
            ReactionType::Unicode(ref x) if x == reactions::STATS => {
                AniListPaginationKind::User(AniListUserView::Stats)
            }
            ReactionType::Unicode(ref x) if x == reactions::FAVOURITES => {
                AniListPaginationKind::User(AniListUserView::Favourites)
            }
            _ => return,
        };
    }

    pub fn user_embed(&self, user: &User) -> CreateEmbed {
        match &self.kind {
            AniListPaginationKind::User(view) => {
                let footer = Some(self.standard_footer());
                match view {
                    AniListUserView::Overview => user_overview_embed(&user, footer),
                    AniListUserView::Stats => user_stats_embed(&user, footer),
                    AniListUserView::Favourites => user_favourites_embed(&user, footer),
                }
            }
            _ => CreateEmbed::default(),
        }
    }

    pub(crate) async fn _user_handler(
        &mut self,
        context: &Context,
        reaction: &Reaction,
    ) -> PaginationResult {
        let user = anilist::client::fetch_user(self.ids[self.cursor]).await?;
        let embed = self.user_embed(&user);
        self.update_message(&context, &reaction, embed).await;

        Ok(())
    }
}
