use crate::models::media::{MediaBase, MediaStatus};

#[inline]
pub fn na_str() -> String {
    "N/A".to_string()
}

#[inline]
pub fn na_long_str() -> String {
    "Not Available".to_string()
}

/// Returns formated time from minutes
pub fn format_time(time_minutes: f64) -> String {
    let minutes = (time_minutes % 60.0).floor();
    let hours = ((time_minutes / 60.0) % 24.0).floor();
    let days = (time_minutes / 60.0 / 24.0).floor();

    if days > 0.0 {
        return format!("{} days, {}:{:02}", days, hours, minutes);
    }

    if hours > 0.0 {
        return format!("{} hours, {} minutes", hours, minutes);
    }

    format!("{} minutes", minutes)
}

pub fn synopsis(text: impl ToString, length: usize) -> String {
    let mut synopsis = text.to_string();
    if synopsis.is_empty() || length == 0 {
        return "N/A".to_string();
    }

    // TODO parse markdown links and images
    // TODO hide/remove spoilers
    // TODO strip html tags

    // TODO a better solution would be to find a extensible
    // markdown parser
    // even if it means making a local nodejs server just to turn
    // anilist markdown to proper markdown that can be accepted by Discord

    synopsis = markdown::parse_markdown_links(synopsis);
    synopsis = markdown::parse_markdown(synopsis);

    let newline_regex = regex::Regex::new("(\n{2,})").unwrap();
    synopsis = newline_regex.replace_all(&synopsis, "").to_string();

    synopsis = synopsis.replace("~~~", "\n");

    if synopsis.len() > length {
        let end = synopsis.char_indices().map(|(i, _)| i).nth(length).unwrap();

        let mut trimmed = synopsis[0..end].to_string();
        trimmed = trimmed.split_at(trimmed.rfind(' ').unwrap()).0.to_string();
        trimmed = markdown::clean_spoilers(trimmed);
        trimmed.push_str(" ...");

        synopsis = trimmed;
    }

    markdown::strip_html_tags(&synopsis).join("")
}

pub fn num_to_emoji(num: u32) -> String {
    match num {
        0 => ":zero:",
        1 => ":one:",
        2 => ":two:",
        3 => ":three:",
        4 => ":four:",
        5 => ":five:",
        6 => ":six:",
        7 => ":seven:",
        8 => ":eight:",
        9 => ":nine:",
        _ => unreachable!("Input should not be a number above 9."),
    }
    .to_string()
}

pub fn media_base_to_legend(media: &[MediaBase]) -> Option<String> {
    let mut statuses = media
        .iter()
        .map(|media| media.status.clone())
        .collect::<Vec<_>>();
    statuses.sort_by(|a, b| b.cmp(&a));
    statuses.dedup_by_key(|status| status.clone());

    let legend: String = statuses
        .iter()
        .map(MediaStatus::to_string_with_emoji)
        .collect::<Vec<_>>()
        .join(" - ");

    Some(legend).filter(String::is_empty)
}

mod markdown {
    use regex::Regex;

    pub(crate) fn clean_spoilers(content: String) -> String {
        let content = content.replace("~!", "||").replace("!~", "||");

        let spoiler_pairs: Vec<_> = content.match_indices("||").collect();

        if spoiler_pairs.len() % 2 != 0 {
            let index = spoiler_pairs[spoiler_pairs.len() - 1].0;
            return content.split_at(index).0.to_string();
        }

        content
    }

    pub(crate) fn parse_markdown(mut content: String) -> String {
        let re = Regex::new(r"(img|webm|youtube)[0-9]{0,3}%?\((.*?)\)").unwrap();

        for cap in re.captures_iter(content.clone().as_str()) {
            match &cap[1] {
                "img" | "webm" => {
                    content = content.replace(&cap[0], format!("[image]({})", &cap[2]).as_str())
                }
                "youtube" => {
                    content = content.replace(&cap[0], format!("[video]({})", &cap[2]).as_str())
                }
                _ => (),
            }
        }

        content
    }

    pub(crate) fn parse_markdown_links(mut content: String) -> String {
        let re = Regex::new(r"\[ (img|webm)[0-9]{0,3}%?\((.*?)\) ]\((.*?)\)").unwrap();

        for cap in re.captures_iter(content.clone().as_str()) {
            content = content.replace(&cap[0], format!("[image link]({})", &cap[3]).as_str());
        }

        content
    }

    use html5ever::rcdom::{Node, NodeData, RcDom};
    use html5ever::tendril::TendrilSink;
    use html5ever::{parse_document, ParseOpts};

    // Code from the dissolve crate
    pub(crate) fn strip_html_tags(input: &str) -> Vec<String> {
        let dom = parse_document(RcDom::default(), ParseOpts::default())
            .from_utf8()
            .one(input.as_bytes());
        let doc = dom.document;
        get_text(&doc)
    }

    /// Helper function to return text in text nodes in pre-order traversal.
    fn get_text(element: &Node) -> Vec<String> {
        match element.data {
            NodeData::Text { ref contents } => {
                let mut text = vec![(&**contents.borrow()).to_owned()];
                for child in &*element.children.borrow() {
                    text.append(&mut get_text(child));
                }
                text
            }
            _ => {
                let mut text = vec![];
                for child in &*element.children.borrow() {
                    text.append(&mut get_text(child));
                }
                text
            }
        }
    }
}
