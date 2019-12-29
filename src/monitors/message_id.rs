use regex::Regex;
use serenity::model::{
    channel::Message,
    id::{ChannelId, GuildId},
};
use serenity::prelude::Context;

use crate::core::consts;

lazy_static! {
    static ref MSG_RE: Regex = Regex::new(r"[0-9]{17,18}").unwrap();
}

pub fn message_id_monitor(context: &Context, message: &Message) {
    let guild_id = match message.guild_id {
        Some(id) => id,
        None => return,
    };

    for cap in MSG_RE.find_iter(message.content.as_str()) {
        if let Ok(msg_id) = cap.as_str().parse::<u64>() {
            match message.channel_id.message(context, msg_id) {
                Ok(msg) => handle_message(context, guild_id, message.channel_id, &msg),
                Err(why) => {
                    warn!("[MessageID Monitor] {}", why);
                    if let Ok(channels) = guild_id.channels(context) {
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
    }
}

fn handle_message(
    context: &Context,
    guild_id: GuildId,
    target_channel_id: ChannelId,
    message: &Message,
) {
    if !message.content.is_empty() || !message.attachments.is_empty() {
        let url = format!(
            "[Jump!](https://discordapp.com/channels/{}/{}/{}/)",
            guild_id, message.channel_id, message.id
        );

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
                .colour(consts::MAIN_COLOUR)
                .description(message.content.clone())
                .field("Original", url, false);

                if !message.attachments.is_empty() && message.attachments[0].dimensions().is_some()
                {
                    e.image(message.attachments[0].url.clone());
                };

                e
            })
        });
    }
}