use crate::core::store::{Pagination, PaginationContainer, PaginationInfo};
use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::{Message, ReactionType};
use serenity::model::id::{MessageId, UserId};
use serenity::prelude::Context;

pub async fn send_embed_message(
    context: &Context,
    message: &Message,
    embed: &CreateEmbed,
    reactions: Vec<ReactionType>,
) -> CommandResult<Message> {
    let sent = message
        .channel_id
        .send_message(&context, |m| {
            m.embed(|e| {
                e.clone_from(&embed);
                e
            })
            .reactions(reactions)
        })
        .await?;

    Ok(sent)
}

pub async fn add_pagination_to_store(
    context: &Context,
    pagination: Box<dyn Pagination>,
    message_id: MessageId,
    author_id: UserId,
) {
    let data = context.data.write().await;
    let container = data.get::<PaginationContainer>().unwrap();
    let pagination_info = PaginationInfo::new(author_id, pagination);
    container.lock().await.insert(message_id, pagination_info);
}
