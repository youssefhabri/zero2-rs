use regex::Regex;
use serenity::builder::CreateEmbed;
use serenity::model::id::{ChannelId, GuildId, MessageId};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use std::collections::HashMap;

use crate::core::consts::MAIN_COLOUR;

lazy_static! {
    pub static ref MESSAGE_ID_RE: Regex = Regex::new(r"[0-9]{17,18}").unwrap();
    pub static ref MESSAGE_LINK_RE: Regex =
        Regex::new(r"https://.*discordapp\.com/channels/([0-9]*)/([0-9]*)/([0-9]*)/?").unwrap();
}

pub async fn id_mention(context: &Context, new_message: &Message) {
    if MESSAGE_LINK_RE.is_match(new_message.content.as_str()) {
        return handle_discord_url(&context, &new_message).await;
    }

    let guild_id = match new_message.guild_id {
        Some(guild_id) => guild_id,
        None => return,
    };

    let message_ids: Vec<MessageId> = MESSAGE_ID_RE
        .find_iter(new_message.content.as_str())
        .filter_map(|cap| cap.as_str().parse::<u64>().ok())
        .map(MessageId)
        .collect();

    for msg_id in message_ids {
        let message = match new_message.channel_id.message(&context, msg_id).await {
            Ok(message) => Some(message),
            Err(_) => find_message_in_guild_channels(&context, guild_id, msg_id).await,
        };

        if let Some(message) = message {
            process_message(&context, guild_id, new_message.channel_id, &message).await;
        }
    }
}

async fn handle_discord_url(context: &Context, message: &Message) {
    if let Some(captures) = MESSAGE_LINK_RE.captures(message.content.as_str()) {
        let guild_id = captures
            .get(1)
            .map(|id| GuildId(id.as_str().parse().unwrap()))
            .unwrap_or_else(|| message.guild_id.unwrap_or(GuildId(0)));

        let channel_id = captures
            .get(2)
            .map(|id| ChannelId(id.as_str().parse().unwrap()))
            .unwrap_or(message.channel_id);

        let message_id: MessageId = captures
            .get(3)
            .map(|id| MessageId(id.as_str().parse().unwrap()))
            .unwrap_or(message.id);

        let target_channel_id = message.channel_id;

        if let Ok(message) = channel_id.message(&context, message_id).await {
            process_message(context, guild_id, target_channel_id, &message).await;
        }
    }
}

async fn find_message_in_guild_channels(
    context: &Context,
    guild_id: GuildId,
    message_id: MessageId,
) -> Option<Message> {
    let channels: HashMap<ChannelId, _> = guild_id.channels(&context).await.ok()?;

    for (channel_id, _) in channels {
        if let Ok(message) = channel_id.message(&context, message_id).await {
            return Some(message);
        }
    }

    None
}

async fn process_message(
    context: &Context,
    target_guild_id: GuildId,
    target_channel_id: ChannelId,
    message: &Message,
) {
    let is_source_channel_nsfw = message.channel(context).await.unwrap().is_nsfw();
    let is_target_channel_nsfw = target_channel_id
        .to_channel(context)
        .await
        .unwrap()
        .is_nsfw();

    if is_source_channel_nsfw && !is_target_channel_nsfw {
        return;
    }

    let message_url = |guild_id, channel_id, message_id| {
        format!(
            "[Jump!](https://discordapp.com/channels/{}/{}/{}/)",
            guild_id, channel_id, message_id
        )
    };

    let url = message_url(target_guild_id, message.channel_id, message.id);
    let datetime = message
        .timestamp
        .format("%a, %B %e, %Y at %H:%M:%S")
        .to_string();

    if !message.embeds.is_empty() {
        let _ = target_channel_id
            .send_message(context, |m| {
                if !message.content.is_empty() {
                    m.content(message.content.clone());
                }

                let embed = message.embeds[0].clone();
                m.embed(|e| {
                    e.clone_from(&CreateEmbed::from(embed));
                    e.field("Original", url, false);
                    e.footer(|f| f.text(datetime));
                    e
                });

                m
            })
            .await;

        return;
    }

    if !message.content.is_empty() || !message.attachments.is_empty() {
        let author = message
            .author
            .nick_in(context, target_guild_id)
            .await
            .unwrap_or_else(|| message.author.name.clone());

        let icon_url = message
            .author
            .avatar_url()
            .unwrap_or_else(|| message.author.default_avatar_url());

        let _ = target_channel_id
            .send_message(context, |m| {
                m.embed(|e| {
                    e.author(|a| a.name(author).icon_url(icon_url))
                        .colour(MAIN_COLOUR)
                        .description(message.content.clone())
                        .footer(|f| f.text(datetime))
                        .field("Original", url, false);

                    if !message.attachments.is_empty() {
                        let mut attachments = message.attachments.clone();
                        if let Some((index, image)) = attachments
                            .iter()
                            .enumerate()
                            .find(|(_, attachment)| attachment.dimensions().is_some())
                        {
                            dbg!(&image);
                            e.image(image.url.clone());
                            attachments.remove(index);
                        }

                        let files = attachments
                            .iter()
                            .map(|item| format!("[{}]({})", item.filename, item.url))
                            .collect::<Vec<String>>();

                        if !files.is_empty() {
                            e.field("Attachments", files.join("\n"), false);
                        }
                    };

                    e
                })
            })
            .await;
    }
}
