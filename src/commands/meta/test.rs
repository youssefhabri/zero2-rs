use serenity::framework::standard::{Args, Command, CommandError};
use serenity::model::channel::Message;
use serenity::prelude::*;

pub struct Test;

impl Command for Test {
    fn execute(&self, _: &mut Context, message: &Message, _: Args) -> Result<(), CommandError> {
        message.channel_id.say("!test");

        Ok(())
    }
}
