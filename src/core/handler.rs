use serenity::prelude::{Context, EventHandler};
use serenity::model::{channel::Message, channel::Reaction, gateway::Activity, gateway::Ready, event::ResumedEvent};

use crate::{menu, monitors};


pub struct Zero2Handler;

impl EventHandler for Zero2Handler {
    fn message(&self, context: Context, message: Message) {
        monitors::run_monitors(&context, &message);
    }

    fn reaction_add(&self, context: Context, add_reaction: Reaction) {
        menu::handle_reaction(&context, &add_reaction);
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(
            Activity::listening("2!help")
        );

        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}