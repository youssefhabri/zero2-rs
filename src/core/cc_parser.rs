use regex::{Captures, Regex};
use serenity::framework::standard::Args;
use serenity::model::{
    channel::Message,
    id::{ChannelId, UserId},
};
use serenity::prelude::Context;

lazy_static! {
    pub static ref CC_RE: Regex = Regex::new(r"\{\{(.*?)\}\}").unwrap();
    pub static ref CC_ARGS_RE: Regex = Regex::new(r"\{\{args?:(.*?)\}\}").unwrap();
}

pub fn parse(context: &Context, message: &Message, args: &Args, cc_content: String) {
    let content = cc_content.clone();
    let arg_caps = CC_ARGS_RE
        .captures_iter(content.as_str())
        .enumerate()
        .collect::<Vec<(usize, Captures)>>();

    let content = if arg_caps.len() > 0 && arg_caps.len() != args.len() {
        format!(
            "Not enough arguments! The command requires {} argument(s)",
            arg_caps.len()
        )
    } else {
        let content = parse_content(cc_content, &message.author.id, &message.channel_id);
        parse_args(content, arg_caps, args)
    };

    let _ = message
        .channel_id
        .send_message(context, |m| m.content(content));
}

fn parse_content(content: String, author_id: &UserId, channel_id: &ChannelId) -> String {
    let mut new_content = content.clone();
    for cap in CC_RE.captures_iter(content.as_str()) {
        let cap_inner: &str = &cap[1];
        let segments = cap_inner.split(':').collect::<Vec<&str>>();

        let id_or_default = |default: &u64| match segments.get(1) {
            Some(id) => id.to_string(),
            None => default.to_string(),
        };

        match segments[0] {
            "author" | "user" => {
                let id = id_or_default(author_id.as_u64());
                new_content = new_content.replace(&cap[0], format!("<@{}>", id).as_str())
            }
            "channel" => {
                let id = id_or_default(channel_id.as_u64());
                new_content = new_content.replace(&cap[0], format!("<#{}>", id).as_str())
            }
            "arg" | "args" => {}
            _ => {}
        }
    }

    new_content
}

fn parse_args(content: String, arg_caps: Vec<(usize, Captures)>, args: &Args) -> String {
    let mut new_content = content.clone();
    let args = args
        .clone()
        .iter::<String>()
        .map(|a| a.unwrap_or_else(|_| "".into()))
        .collect::<Vec<String>>();

    for (idx, cap) in arg_caps {
        let cap = cap.get(0).map_or("", |m| m.as_str());
        new_content = new_content.replace(cap, args[idx].as_str());
    }

    new_content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_user() {
        assert_eq!(
            parse_content("Hello, {{user:10}}!".to_string(), &UserId(0), &ChannelId(0)),
            "Hello, <@10>!".to_string()
        );
        assert_eq!(
            parse_content("Hello, {{user}}!".to_string(), &UserId(0), &ChannelId(0)),
            "Hello, <@0>!".to_string()
        );
    }

    #[test]
    fn test_parse_author() {
        assert_eq!(
            parse_content("Hello, {{author}}!".to_string(), &UserId(0), &ChannelId(0)),
            "Hello, <@0>!".to_string()
        );
    }

    #[test]
    fn test_parse_channel() {
        assert_eq!(
            parse_content(
                "Hello, {{channel:200}}!".to_string(),
                &UserId(0),
                &ChannelId(0)
            ),
            "Hello, <#200>!".to_string()
        );
        assert_eq!(
            parse_content("Hello, {{channel}}!".to_string(), &UserId(0), &ChannelId(0)),
            "Hello, <#0>!".to_string()
        );
    }
}
