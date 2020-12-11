use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

lazy_static! {
    static ref COOKIES: Option<Vec<String>> = load_from_url(
        "https://raw.githubusercontent.com/ianli/fortune-cookies-galore/master/fortunes.txt"
    );
    static ref FORTUNES: Option<Vec<String>> = load_from_url(
        "https://raw.githubusercontent.com/larryprice/fortune-cookie-api/master/data/proverbs.txt"
    );
}

fn load_from_url(url: &str) -> Option<Vec<String>> {
    let response = reqwest::blocking::get(url).ok()?;
    let text = response.text().ok()?;
    let cookies: Vec<String> = text.split('\n').map(|s| s.to_string()).collect();

    Some(cookies)
}

fn random_from_vec<'a, T: Clone>(list: &'a Vec<T>) -> &'a T {
    let random_number = crate::utils::random_number(0, list.len() - 1);
    &list[random_number]
}

#[command]
async fn cookie(context: &Context, message: &Message, _args: Args) -> CommandResult {
    let cookies = tokio::task::spawn_blocking(|| COOKIES.as_ref())
        .await?
        .ok_or(CommandError::from("COOKIES is None"))?;

    let title = format!("{}'s fortune cookie!", message.author.name);
    let description = random_from_vec(&cookies).trim();

    let _ = message
        .channel_id
        .send_message(&context, |m| {
            m.embed(|e| e.colour(16711769).title(title).description(description))
        })
        .await;

    Ok(())
}

#[command]
async fn fortune(context: &Context, message: &Message, _args: Args) -> CommandResult {
    let fortunes = tokio::task::spawn_blocking(|| FORTUNES.as_ref())
        .await?
        .ok_or(CommandError::from("FORTUNES is None"))?;

    let title = format!("{}'s fortune!", message.author.name);
    let description = random_from_vec(&fortunes).trim();

    let _ = message
        .channel_id
        .send_message(&context, |m| {
            m.embed(|e| e.colour(16711769).title(title).description(description))
        })
        .await;

    Ok(())
}
