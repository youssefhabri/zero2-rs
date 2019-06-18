use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::menu;

use std::thread;
use std::time::Duration;

#[command]
fn test(context: &mut Context, message: &Message, _args: Args) -> CommandResult {
    let sending = message.channel_id.send_message(&context.http, |m| {
        m.content("Test message!")
            .reactions(menu::reactions::default())
    });

    if let Ok(msg) = sending {
        delete_reactions_after_delay(context.clone(), msg, 5 * 1000);
    }

    Ok(())
}

fn delete_reactions_after_delay(context: Context, message: Message, delay: u64) {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(delay));
        let _ = message.delete_reactions(&context);
    });
}
