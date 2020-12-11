use async_trait::async_trait;
use serenity::model::prelude::{GuildId, Member, Message, Reaction, Ready, ResumedEvent};
use serenity::prelude::{Context, EventHandler};

use crate::monitors;

pub struct Zero2EventHandler;

#[async_trait]
impl EventHandler for Zero2EventHandler {
    async fn guild_member_addition(&self, context: Context, guild_id: GuildId, new_member: Member) {
        monitors::new_member_monitor(&context, guild_id, &new_member).await;
    }

    async fn message(&self, context: Context, new_message: Message) {
        monitors::message_monitor(&context, &new_message).await;
    }

    async fn reaction_add(&self, context: Context, reaction: Reaction) {
        monitors::reaction_add_monitor(&context, &reaction).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
