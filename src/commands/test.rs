use serenity::framework::standard::{macros::command, Args, CommandResult, Delimiter};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::thread;
use std::time::Duration;

use crate::core::cc_parser;
use crate::menu;

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

#[command]
#[owners_only]
fn eval(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let segments = args.message().split('|').collect::<Vec<&str>>();
    let cc_content = segments[0].to_string();

    let cc_args = if let Some(args_string) = segments.get(1) {
        Args::new(args_string, &[Delimiter::Single(' ')])
    } else {
        args
    };

    cc_parser::parse(&context, &message, &cc_args, cc_content);

    Ok(())
}
