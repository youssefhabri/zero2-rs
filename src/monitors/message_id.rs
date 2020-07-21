use serenity::builder::CreateEmbed;
use serenity::model::{
    channel::Message,
    id::{ChannelId, GuildId, MessageId},
};
use serenity::prelude::Context;

use crate::core::consts::{MAIN_COLOUR, MESSAGE_ID_RE, MESSAGE_LINK_RE};

fn message_url(guild_id: GuildId, channel_id: ChannelId, message_id: MessageId) -> String {
    format!(
        "[Jump!](https://discordapp.com/channels/{}/{}/{}/)",
        guild_id, channel_id, message_id
    )
}

fn handle_message_url(context: &Context, message: &Message) {
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

        if let Ok(message) = context
            .http
            .get_message(*channel_id.as_u64(), *message_id.as_u64())
        {
            handle_message(context, guild_id, target_channel_id, &message);
        }
    }
}

pub fn message_id_monitor(context: &Context, message: &Message) {
    if MESSAGE_LINK_RE.is_match(message.content.as_str()) {
        return handle_message_url(context, message);
    }

    let guild_id = match message.guild_id {
        Some(id) => id,
        None => return,
    };

    for cap in MESSAGE_ID_RE.find_iter(message.content.as_str()) {
        let msg_id: u64 = match cap.as_str().parse::<u64>() {
            Ok(msg_id) => msg_id,
            Err(_) => return,
        };

        match message.channel_id.message(context, msg_id) {
            Ok(msg) => handle_message(context, guild_id, message.channel_id, &msg),
            Err(why) => {
                warn!("[MessageID Monitor] {}", why);
                let channels = match guild_id.channels(context) {
                    Ok(channels) => channels,
                    Err(_) => continue,
                };

                for (channel_id, _) in channels {
                    match channel_id.message(context, msg_id) {
                        Ok(msg) => {
                            handle_message(context, guild_id, message.channel_id, &msg);
                            break;
                        }
                        Err(why) => warn!("[MessageID Monitor] {}", why),
                    }
                }
            }
        }
    }
}

fn handle_message(
    context: &Context,
    guild_id: GuildId,
    target_channel_id: ChannelId,
    message: &Message,
) {
    let is_message_channel_nsfw = message.channel(context).unwrap().is_nsfw();
    let is_target_channel_nsfw = target_channel_id.to_channel(context).unwrap().is_nsfw();

    // Make sure that the bot doesn't post messages from nsfw channels to sfw channels
    if is_message_channel_nsfw && !is_target_channel_nsfw {
        return;
    }

    let url = message_url(guild_id, message.channel_id, message.id);

    if !message.embeds.is_empty() {
        let _ = target_channel_id.send_message(context, |m| {
            if !message.content.is_empty() {
                m.content(message.content.clone());
            }

            let embed = message.embeds[0].clone();
            m.embed(|e| {
                e.clone_from(&CreateEmbed::from(embed));
                e.field("Original", url, false);
                e
            });

            m
        });

        return;
    }

    if !message.content.is_empty() || !message.attachments.is_empty() {
        let _ = target_channel_id.send_message(context, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(
                        message
                            .author
                            .nick_in(context, guild_id)
                            .unwrap_or_else(|| message.author.name.clone()),
                    )
                    .icon_url(
                        message
                            .author
                            .avatar_url()
                            .unwrap_or_else(|| message.author.default_avatar_url()),
                    )
                })
                .colour(MAIN_COLOUR)
                .description(message.content.clone())
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
        });
    }
}
