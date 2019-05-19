use serenity::model::{
    channel::Message, channel::Reaction, event::ResumedEvent, gateway::Activity, gateway::Ready,
    guild::Member, id::GuildId,
};
use serenity::prelude::{Context, EventHandler};

use crate::{menu, monitors};

pub struct Zero2Handler;

impl EventHandler for Zero2Handler {
    fn guild_member_addition(&self, context: Context, guild_id: GuildId, new_member: Member) {
        monitors::new_member_monitors(&context, guild_id, &new_member);
    }

    fn message(&self, context: Context, message: Message) {
        monitors::message_monitors(&context, &message);
    }

    fn reaction_add(&self, context: Context, add_reaction: Reaction) {
        menu::handle_reaction(&context, &add_reaction);
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::listening("2!help"));

        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
