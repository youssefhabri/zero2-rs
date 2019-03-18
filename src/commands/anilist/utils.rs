use dissolve::strip_html_tags;
use regex::Regex;

fn _remove_spoilers(content: &String) -> String {
    let re = Regex::new(r"<span class='markdown_spoiler'>(.+?)</span>").unwrap();

    let mut result = String::new();

    for cap in re.captures_iter(content.as_str()) {
        let capture = strip_html_tags(&cap[0]).join("");
        result = content.replace(capture.as_str(), "");
    }

    result
}

fn clean_spoilers(content: String) -> String {
    let content = content
        .replace("~!", "||")
        .replace("!~", "||");

    let spoiler_pairs: Vec<_> = content.match_indices("||").collect();

    let result = if spoiler_pairs.len() % 2 != 0 {
        let index = spoiler_pairs[spoiler_pairs.len() - 1].0;
        content.split_at(index).0.to_string()
    } else {
        content
    };

    result
}

pub fn synopsis(description: &String, length: usize) -> String {

    let synopsis = description.clone();
//    synopsis = synopsis.replace("\n\n", "\n");

    if synopsis.len() > length {
        // Slicing by nth character rather and a simple index
        let end = synopsis.char_indices().map(|(i, _)| i).nth(length).unwrap();

        let mut result = (&synopsis[0..end]).to_string();
        result = result.split_at(result.rfind(" ").unwrap()).0.to_string();
        result = clean_spoilers(result);

        return format!("{} ...", result);
    }

    if !synopsis.is_empty() { synopsis } else { "N/A".into() }
}

#[cfg(test)]
mod tests {
    use crate::commands::anilist::utils::*;

    #[test]
    fn test_remove_spoilers() {
        let content = "<p>Testing spoiler removing function. </p>\n<p><span class='markdown_spoiler'><span>This is a spoiler.</span></span></p>";
        let content_no_spoilers = "<p>Testing spoiler removing function. </p>\n<p><span class=\'markdown_spoiler\'><span></span></span></p>";
        assert_eq!(remove_spoilers(&content.to_string()), content_no_spoilers);
    }

    #[test]
    fn test_synopsis() {
        let content = "_Birthday:__ July 7\n__Hair Color:__ Purple\n__Eye Color:__ Blue\n__Height:__ 165 cm\n__Weight:__ 45 kg\n~!5 kg when she was under the influence of the heavy crab.!~\n\nHitagi, the main female character, is a weak-looking girl with an \"incurable disease\". She is in the same class as Koyomi, but he has almost never heard her speak. When she was in the first year of high school, she encountered a mysterious crab, after which she became weightless. Ever since, she has avoided contact with everyone else, threatening everyone who discovers her secret. She called herself a tsundere and always speaks in an abusive style.\n\n~!At the end of Mayoi Snail, she admits that she loves Koyomi, and subsequently enters into a relationship with him.!~\n\n~!After the events in Tsukihi Phoenix, she overcame all of her trauma, and became a rather cheerful and normal girl. She starts calling Araragi with a cute nickname, chuckling at small things and sending e-mails full of Emojis, although her sharp tongue is still there, but toned down.!~\n\n~! She seems to also have a father complex as she becomes excited to the point of being unable to fall asleep when using her father's blanket.!~";
        let expected = "Test test test ||hello spoilers|| test test.  ...";
        assert_eq!(synopsis(&content.to_string(), 55), expected);
    }
}