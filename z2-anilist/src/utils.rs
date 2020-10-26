use crate::models::media::MediaBase;

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
    } else if hours > 0.0 {
        return format!("{} hours, {} minutes", hours, minutes);
    }

    format!("{} minutes", minutes)
}

pub fn synopsis(text: impl ToString, length: usize) -> String {
    let synopsis = text.to_string();
    if synopsis.is_empty() || length == 0 {
        return "N/A".to_string();
    }

    // TODO parse markdown links and images

    if synopsis.len() > length {
        let end = synopsis.char_indices().map(|(i, _)| i).nth(length).unwrap();

        let mut trimmed = synopsis[0..end].to_string();
        trimmed = trimmed.split_at(trimmed.rfind(' ').unwrap()).0.to_string();
        trimmed.push_str(" ...");

        // TODO hide/remove spoilers
        // TODO strip html tags

        return trimmed;
    }

    synopsis
}

pub fn num_to_emoji(num: u32) -> String {
    match num {
        0 => ":zero:".to_string(),
        1 => ":one:".to_string(),
        2 => ":two:".to_string(),
        3 => ":three:".to_string(),
        4 => ":four:".to_string(),
        5 => ":five:".to_string(),
        6 => ":six:".to_string(),
        7 => ":seven:".to_string(),
        8 => ":eight:".to_string(),
        9 => ":nine:".to_string(),
        _ => num.to_string(),
    }
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
        .map(|status| status.to_string_with_emoji())
        .collect::<Vec<_>>()
        .join(" - ");

    if !legend.is_empty() {
        return Some(legend);
    }

    None
}
