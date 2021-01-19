use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::{Message, Reaction};
use serenity::prelude::Context;
use serenity::utils::Colour;
use urbandictionary::model::Definition;

use crate::types::{Pagination, PaginationResult};
use crate::{reactions, utils};

pub struct UrbanDictionaryPagination {
    pub(crate) definitions: Vec<Definition>,
    pub(crate) cursor: usize,
    pub(crate) prev_cursor: usize,
}

impl UrbanDictionaryPagination {
    pub async fn init(
        context: &Context,
        message: &Message,
        definitions: Vec<Definition>,
    ) -> CommandResult {
        let first = definitions[0].clone();
        let num = definitions.len();
        let pagination = UrbanDictionaryPagination {
            definitions,
            cursor: 0,
            prev_cursor: 0,
        };
        let embed = urban_dictionary_embed(&first, pagination.footer());
        let reactions = reactions::default(num);
        let sent =
            utils::send_embed_message(context, &message.channel_id, &embed, reactions).await?;
        utils::add_pagination_to_store(context, pagination, sent.id, message.author.id).await;

        Ok(())
    }

    fn footer(&self) -> Option<String> {
        if self.definitions.len() > 1 {
            return Some(format!("Page: {}/{} | ", self.cursor() + 1, self.count()));
        }

        None
    }
}

#[async_trait::async_trait]
impl Pagination for UrbanDictionaryPagination {
    async fn handle(&mut self, context: &Context, reaction: &Reaction) -> PaginationResult {
        if self.cursor == self.prev_cursor {
            return Ok(());
        }

        let definition = &self.definitions[self.cursor];
        let embed = urban_dictionary_embed(&definition, self.footer());

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
        self.definitions.len()
    }

    fn cursor(&self) -> usize {
        self.cursor
    }

    fn set_cursor(&mut self, value: usize) {
        self.prev_cursor = self.cursor;
        self.cursor = value;
    }
}

fn urban_dictionary_embed(definition: &Definition, footer: Option<String>) -> CreateEmbed {
    let mut s = definition.definition.clone();
    if s.len() > 1800 {
        s.truncate(1800);
    }

    let mut fields = Vec::new();
    let example = definition.example.clone();
    if !example.is_empty() {
        fields.push(("Example", example, true));
    }

    let votes = format!(
        "👍: **{}** 👎: **{}**",
        &definition.thumbs_up, &definition.thumbs_down
    );
    fields.push(("Votes", votes, true));

    CreateEmbed::default()
        .color(Colour::GOLD)
        .title(&format!("Definition of {}", &definition.word))
        .url(&definition.permalink)
        .description(s)
        .footer(|f| f
            .icon_url("https://d2gatte9o95jao.cloudfront.net/assets/apple-touch-icon-1734beeaa059fbc5587bddb3001a0963670c6de8767afb6c67d88d856b0c0dad.png")
            .text(&format!("{}Defined by {}", footer.unwrap_or_default(), definition.author)))
            .fields(fields)
            .to_owned()
}
