use regex::Regex;
use serenity::model::{channel::Message, guild::Member, id::GuildId};
use serenity::prelude::Context;

use crate::core::consts::PREFIX;

mod anilist;

lazy_static! {
    static ref MSG_RE: Regex = Regex::new(r"[0-9]{17,18}").unwrap();
}

pub fn message_monitors(context: &Context, message: &Message) {
    if !message.author.bot
        && !message
            .content_safe(&context.cache)
            .as_str()
            .starts_with(PREFIX.as_str())
    {
        anilist::anilist_links_monitor(context, message);
        //anilist::rem_monitor(&context, &message);
        message_id_monitor(context, message);
    }
}

pub fn new_member_monitors(context: &Context, guild_id: GuildId, new_member: &Member) {
    // Greet the user?
}

pub fn message_id_monitor(context: &Context, message: &Message) {
    if let Some(cap) = MSG_RE.find(message.content.as_str()) {
        if let Ok(msg_id) = cap.as_str().parse::<u64>() {
            if let Ok(msg) = message.channel_id.message(context, msg_id) {
                if !msg.content.is_empty() {
                    let guild_id = match message.guild_id {
                        Some(id) => id,
                        None => return,
                    };

                    let url = format!(
                        "[Jump!](https://discordapp.com/channels/{}/{}/{}/)",
                        guild_id.as_u64(),
                        msg.channel_id.as_u64(),
                        msg.id.as_u64()
                    );

                    let _ = message.channel_id.send_message(context, |m| {
                        m.embed(|e| {
                            e.author(|a| {
                                a.name(
                                    msg.author
                                        .nick_in(context, guild_id)
                                        .unwrap_or_else(|| msg.author.name.clone()),
                                )
                                .icon_url(msg.author.default_avatar_url())
                            })
                            .description(msg.content)
                            .field("Original", url, false)
                        })
                    });
                }
            }
        }
    }
}
