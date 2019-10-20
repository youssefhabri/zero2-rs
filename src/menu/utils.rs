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
        if let Err(why) = channel_id.edit_message(&context, message_id, |m| {
            m.embed(|e| {
                e.clone_from(&embed_content);
                e
            })
        }) {
            warn!("Err editing message: {:?}", why);
        }
    }
}

pub fn get_page_content(
    context: &Context,
    message_id: MessageId,
    page: u32,
) -> Option<CreateEmbed> {
    let data = context.data.read();

    //    let paginator = data.get_mut::<MessagePaginator>().unwrap();
    let paginator = data.get::<MessagePaginator>().unwrap();

    let pagination = match paginator.get(&message_id) {
        Some(pagination) => pagination,
        None => return None,
    };

    Some(pagination.pages[page as usize].clone())
}
