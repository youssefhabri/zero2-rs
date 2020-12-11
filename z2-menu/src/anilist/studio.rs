use anilist::models::Studio;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::{Message, Reaction};
use serenity::prelude::Context;

use crate::anilist::embeds::studio_embed;
use crate::anilist::{AniListPagination, AniListPaginationKind};
use crate::types::PaginationResult;
use crate::{reactions, utils};

impl AniListPagination {
    pub async fn new_studio_pagination(
        context: &Context,
        message: &Message,
        studios: &[Studio],
    ) -> CommandResult {
        let ids = studios.iter().map(|studio| studio.id).collect();
        let kind = AniListPaginationKind::Studio;
        let pagination = AniListPagination::new(ids, kind);

        let embed = pagination.studio_embed(&studios[0]);
        let reactions = reactions::default(studios.len());
        let sent = utils::send_embed_message(&context, &message, &embed, reactions).await?;

        utils::add_pagination_to_store(&context, pagination, sent.id, message.author.id).await;

        Ok(())
    }

    pub fn studio_embed(&self, studio: &Studio) -> CreateEmbed {
        let footer = Some(self.standard_footer());
        studio_embed(studio, footer)
    }

    pub(crate) async fn _studio_handler(
        &mut self,
        context: &Context,
        reaction: &Reaction,
    ) -> PaginationResult {
        let studio = anilist::client::fetch_studio(self.ids[self.cursor]).await?;
        let embed = self.studio_embed(&studio);
        self.update_message(&context, &reaction, embed).await;

        Ok(())
    }
}
