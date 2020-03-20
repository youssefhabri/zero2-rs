use regex::Regex;
use serenity::http::AttachmentType;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::fs::File;
use std::path::PathBuf;

use crate::core::consts::EMOJIS;
use crate::models::emoji::Emoji;

// emoji text regex = ^:(.+):$
// emoji text/id regex = <:(.+):([0-9]+)>

lazy_static! {
    static ref NITRO_EMOJI_RE: Regex = Regex::new(r"^<:(.+):([0-9]+)>$").unwrap();
    static ref NO_NITRO_EMOJI_RE: Regex = Regex::new(r"^:(.+):$").unwrap();
}

pub fn emojis_monitor(context: &Context, message: &Message) {
    if NITRO_EMOJI_RE.is_match(&message.content) {
        return;
    }

    if !NO_NITRO_EMOJI_RE.is_match(&message.content) {
        return;
    }

    let emoji = message.content.clone().replace(':', "");

    if let Some(emoji) = EMOJIS.get(&emoji) {
        let file = get_emoji_file(&emoji);
        let _ = message.channel_id.send_files(
            context,
            vec![AttachmentType::Path(file.as_path())],
            |m| m,
        );
    }
}

fn get_emoji_file(emoji: &Emoji) -> PathBuf {
    let file_ext: &str = emoji.image.split('.').last().unwrap_or("png");

    let path = PathBuf::from(format!(
        "assets/emojis/{}.{}",
        emoji.title.clone(),
        file_ext
    ));

    if path.exists() {
        return path;
    }

    let mut new_file = File::create(path.clone()).unwrap();
    let mut res = reqwest::blocking::get(&emoji.image).unwrap();

    std::io::copy(&mut res, &mut new_file);

    path
}
