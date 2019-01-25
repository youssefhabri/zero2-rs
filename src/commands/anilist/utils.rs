use dissolve::strip_html_tags;
use math::round::floor;

pub fn synopsis(description: &String, length: usize) -> String {
    let mut synopsis = strip_html_tags(description.as_str()).join(" ");
    synopsis = synopsis.replace("\n\n", "\n");

    let segments: Vec<&str> = synopsis.split("\n").collect();

    let slices = if segments.len() > 5 {
        &segments[..5]
    } else {
        segments.as_slice()
    };
    synopsis = slices.join("\n");

    if synopsis.len() > length {
        return format!("{} ...", &synopsis[..length]);
    }

    synopsis
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