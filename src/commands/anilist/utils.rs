use dissolve::strip_html_tags;
use math::round::floor;
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

pub fn format_time(time_minutes: f64) -> String {
    let minutes = floor(time_minutes % 60.0, 0);
    let hours = floor((time_minutes / 60.0) % 24.0, 0);
    let days = floor(time_minutes / 60.0 / 24.0, 0);

    if days > 0.0 {
        return format!("{} days, {}:{}", days, hours, minutes);
    } else if hours > 0.0 {
        return format!("{} hours, {} minutes", hours, minutes);
    }

    format!("{} minutes", minutes)
}