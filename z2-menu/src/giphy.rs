use serde::Deserialize;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::{Message, Reaction};
use serenity::prelude::Context;

use crate::types::{Pagination, PaginationResult};
use crate::{reactions, utils};

#[derive(Clone, Deserialize, Debug)]
pub struct GiphyImageOriginal {
    pub url: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct GiphyImages {
    pub original: GiphyImageOriginal,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Giphy {
    pub url: String,
    pub title: String,
    pub images: GiphyImages,
}

#[derive(Deserialize, Debug)]
pub struct GiphyResponse {
    pub data: Vec<Giphy>,
}

pub struct GiphyPagination {
    pub(crate) gifs: Vec<Giphy>,
    pub(crate) cursor: usize,
    pub(crate) prev_cursor: usize,
}

impl GiphyPagination {
    pub async fn init(context: &Context, message: &Message, gifs: Vec<Giphy>) -> CommandResult {
        let first = gifs[0].clone();
        let num = gifs.len();
        let pagination = GiphyPagination {
            gifs,
            cursor: 0,
            prev_cursor: 0,
        };
        let embed = giphy_embed(&first, pagination.footer());
        let reactions = reactions::default(num);
        let sent =
            utils::send_embed_message(&context, &message.channel_id, &embed, reactions).await?;
        utils::add_pagination_to_store(&context, pagination, sent.id, message.author.id).await;

        Ok(())
    }

    fn footer(&self) -> Option<String> {
        if self.gifs.len() > 1 {
            return Some(format!("Page: {}/{} | ", self.cursor() + 1, self.count()));
        }

        None
    }
}

#[async_trait::async_trait]
impl Pagination for GiphyPagination {
    async fn handle(&mut self, context: &Context, reaction: &Reaction) -> PaginationResult {
        if self.cursor == self.prev_cursor {
            return Ok(());
        }

        let gif = &self.gifs[self.cursor];
        let embed = giphy_embed(&gif, self.footer());

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

        Ok(())
    }

    fn count(&self) -> usize {
        self.gifs.len()
    }

    fn cursor(&self) -> usize {
        self.cursor
    }

    fn set_cursor(&mut self, value: usize) {
        self.prev_cursor = self.cursor;
        self.cursor = value;
    }
}

fn giphy_embed(gif: &Giphy, footer: Option<String>) -> CreateEmbed {
    CreateEmbed::default()
        .color(3447003)
        .title(&gif.title)
        .url(&gif.url)
        .image(&gif.images.original.url)
        .footer(|f| {
            f.icon_url("https://giphy.com/static/img/giphy_logo_square_social.png")
                .text(format!("{}Powered by Giphy", footer.unwrap_or_default()))
        })
        .to_owned()
}
