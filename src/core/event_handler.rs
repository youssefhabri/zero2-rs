use async_trait::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::{Activity, Member, Message, Reaction, Ready, ResumedEvent};
use serenity::prelude::{Context, EventHandler};

use crate::monitors;

pub struct Zero2EventHandler;

#[async_trait]
impl EventHandler for Zero2EventHandler {
    async fn guild_member_addition(&self, context: Context, new_member: Member) {
        monitors::new_member_monitor(&context, &new_member).await;
    }

    async fn message(&self, context: Context, new_message: Message) {
        monitors::message_monitor(&context, &new_message).await;
    }

    async fn reaction_add(&self, context: Context, reaction: Reaction) {
        monitors::reaction_add_monitor(&context, &reaction).await;
    }

    async fn ready(&self, context: Context, ready: Ready) {
        context.set_activity(Activity::listening("!!help")).await;

        println!("Connected as {}", ready.user.name);

        for guild in ready.guilds {
            let guild_id = guild.id;

            match guild_id.to_guild_cached(&context) {
                Some(guild) => println!("[GUILD] Available in {}", guild.name),
                None => error!("Guild not found in cache"),
            }

            if let Err(why) = interactions::register_interactions(&context, guild_id).await {
                error!("Error registering interactions: {}", why);
            }
        }
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        interactions::handle_interaction_create(&context, interaction).await;
    }
}
