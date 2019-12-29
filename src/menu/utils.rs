use crate::core::store::MessagePaginator;
use serenity::builder::CreateEmbed;
use serenity::model::id::{ChannelId, MessageId};
use serenity::prelude::Context;

pub fn update_message(
    context: &Context,
    channel_id: ChannelId,
    message_id: MessageId,
    embed_content: Option<CreateEmbed>,
) {
    if let Some(embed_content) = embed_content {
        let edit_result = channel_id.edit_message(&context, message_id, |m| {
            m.embed(|e| {
                e.clone_from(&embed_content);
                e
            })
        });

        if let Err(why) = edit_result {
            error!("Error editing message: {:?}", why);
        }
    }
}

pub fn get_page_content(
    context: &Context,
    message_id: MessageId,
    page: u32,
) -> Option<CreateEmbed> {
    let data = context.data.read();
    let paginator = data.get::<MessagePaginator>().unwrap();

    match paginator.get(&message_id) {
        Some(pagination) => Some(pagination.pages[page as usize].clone()),
        None => None,
    }
}
