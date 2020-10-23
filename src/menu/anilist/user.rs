use anilist::models::User;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::{Message, Reaction, ReactionType};
use serenity::prelude::Context;

use super::embeds::{user_favourites_embed, user_overview_embed, user_stats_embed};
use super::types::{AniListPaginationKind, AniListUserView};
use super::AniListPagination;
use crate::core::store::Pagination;
use crate::menu::{reactions, utils};

impl AniListPagination {
    pub async fn new_user_pagination(
        context: &Context,
        message: &Message,
        users: &[User],
        view: AniListUserView,
    ) -> CommandResult {
        let ids = users.iter().map(|user| user.id).collect();
        let kind = AniListPaginationKind::User(view);
        let pagination = AniListPagination::new(ids, kind);

        let embed = pagination.user_embed(&users[0]);
        let reactions = reactions::user(users.len());
        let sent = utils::send_embed_message(&context, &message, &embed, reactions).await?;

        utils::add_pagination_to_store(&context, Box::new(pagination), sent.id, message.author.id)
            .await;

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

    fn user_footer(&self) -> String {
        let footer = "Powered by AniList".to_string();

        // Page: 1/6 | Powered by AniList
        if self.ids.len() > 1 {
            return format!("Page: {}/{} | {}", self.cursor() + 1, self.len(), footer);
        }

        footer
    }

    pub fn user_embed(&self, user: &User) -> CreateEmbed {
        match &self.kind {
            AniListPaginationKind::User(view) => {
                let footer = Some(self.user_footer());
                match view {
                    AniListUserView::Overview => user_overview_embed(&user, footer),
                    AniListUserView::Stats => user_stats_embed(&user, footer),
                    AniListUserView::Favourites => user_favourites_embed(&user, footer),
                }
            }
            _ => CreateEmbed::default(),
        }
    }

    pub(crate) async fn _user_handler(&mut self, context: &Context, reaction: &Reaction) {
        let response = anilist::client::fetch_user(self.ids[self.cursor]).await;
        let user = match response {
            Ok(user) => user,
            Err(why) => {
                println!("UserFetch error: {}", why);
                return;
            }
        };

        let embed = self.user_embed(&user);
        self.update_message(&context, &reaction, embed).await;
    }
}
