use dissolve::strip_html_tags;
use regex::Regex;

// TODO Refactor markdown code or break into own module

fn clean_spoilers(content: String) -> String {
    let content = content.replace("~!", "||").replace("!~", "||");

    let spoiler_pairs: Vec<_> = content.match_indices("||").collect();

    if spoiler_pairs.len() % 2 != 0 {
        let index = spoiler_pairs[spoiler_pairs.len() - 1].0;
        return content.split_at(index).0.to_string();
    }

    content
}

fn parse_markdown(mut content: String) -> String {
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

fn parse_markdown_links(mut content: String) -> String {
    let re = Regex::new(r"\[ (img|webm)[0-9]{0,3}%?\((.*?)\) ]\((.*?)\)").unwrap();

    for cap in re.captures_iter(content.clone().as_str()) {
        content = content.replace(&cap[0], format!("[image link]({})", &cap[3]).as_str());
    }

    content
}

pub fn synopsis(description: &str, length: usize) -> String {
    let mut synopsis = description.to_string();

    synopsis = parse_markdown_links(synopsis);
    synopsis = parse_markdown(synopsis);

    if synopsis.len() > length {
        // Slicing by nth character rather and a simple index
        let end = synopsis.char_indices().map(|(i, _)| i).nth(length).unwrap();

        let mut result = (&synopsis[0..end]).to_string();
        result = result.split_at(result.rfind(' ').unwrap()).0.to_string();
        result = clean_spoilers(result);

        synopsis = format!("{} ...", result);
    }

    if !synopsis.is_empty() {
        strip_html_tags(synopsis.as_str()).join("")
    } else {
        "N/A".into()
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::anilist::utils::*;

    #[test]
    fn test_synopsis() {
        let content = "This is a test post. Please ignore.\n[ img100(https://66.media.tumblr.com/c96c0139755c00d0b9fb7dbae51208cf/tumblr_pftji62LCI1x2kdwmo2_r3_500.gif) ](http://google.com)";
        let expected = "Test test test ||hello spoilers|| test test.  ...";
        assert_eq!(synopsis(&content.to_string(), 300), expected);
    }

    #[test]
    fn test_clean_markdown_links() {
        let content = "This is a test post. Please ignore.\n[image link](http://google.com)";
        assert_eq!(
            parse_markdown_links(content.to_string()),
            "This is a test post. Please ignore.\n[image link](http://google.com)"
        );
    }

    #[test]
    fn test_clean_markdown() {
        let content = "This is a test post. Please ignore.\n[image](http://google.com)";
        assert_eq!(
            parse_markdown(content.to_string()),
            "This is a test post. Please ignore.\n[image](http://google.com)"
        );
    }
}
