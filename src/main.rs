use fern::colors::{
    Color,
    ColoredLevelConfig
};

use zero2_rs::Zero2Client;


fn main() {
    fern_setup().expect("Failed to apply fern settings.");

    let mut client = Zero2Client::new();

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

// Code from momiji-rust bot
fn fern_setup() -> Result<(), log::SetLoggerError> {
    // This is a bit verbose, but it allows for logging to console with colors and to a file
    // without to avoid ANSI color codes showing up in the log. This is mostly to improve
    // visibility.
    let colors = ColoredLevelConfig::new()
        .trace(Color::Magenta)
        .debug(Color::Cyan)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    let term_out = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{time}  {level:level_width$}{target:target_width$}> {msg}",
                time = chrono::Utc::now().format("%F %T"),
                level = colors.color(record.level()),
                target = format!("{}:{}", record.target(), record.line().unwrap_or(0)),
                msg = message,
                level_width = 8,
                target_width = 60
            ))
        })
        .chain(std::io::stdout())
        .into_shared();

    let file_out = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{time}  {level:level_width$}{target:target_width$}> {msg}",
                time = chrono::Utc::now().format("%F %T"),
                level = record.level(),
                target = format!("{}:{}", record.target(), record.line().unwrap_or(0)),
                msg = message,
                level_width = 8,
                target_width = 60
            ))
        })
        .chain(fern::log_file("output.log").expect("Failed to load log file"))
        .into_shared();

    fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .level_for("serenity", log::LevelFilter::Debug)
        .level_for("zero2_rs", log::LevelFilter::Debug)
        .level_for("html5ever", log::LevelFilter::Off)
        .chain(term_out)
        .chain(file_out)
        .apply()
}