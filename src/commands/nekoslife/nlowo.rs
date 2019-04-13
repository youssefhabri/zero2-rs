use serenity::framework::standard::{Args, Command, CommandError};
use serenity::model::channel::Message;
use serenity::prelude::*;
use regex::Regex;
use rand::prelude::*;

pub struct NLOwOCommand;

impl Command for NLOwOCommand {
    fn execute(&self, _: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {
        if args.full().is_empty() {
            let _ = message.channel_id.say("You need to input text to convert.");
            return Ok(());
        }

        let mut rnd = rand::thread_rng();

        let faces = [" owo ", " UwU ", " >w< ", " ^w^ "];
        let mut text = args.full()
            .replace("ove", "uv")
            .replace("n", "ny")
            .replace("N", "NY")
            .to_owned();
        text = Regex::new(r"[rl]").unwrap().replace_all(text.as_str(), "w").into();
        text = Regex::new(r"[RL]").unwrap().replace_all(text.as_str(), "W").into();
        text = Regex::new(r"[!]").unwrap().replace_all(text.as_str(), faces[rnd.gen_range(0, 3)]).into();

        let _ = message.delete();
        let _ = message.channel_id.send_message(|m| m
            .content(format!("<@{}> said: {}", message.author.id, text))
        );

        Ok(())
    }
}