use chrono::Utc;
use serenity::model::{
    channel::Reaction, event::PresenceUpdateEvent, event::ResumedEvent, gateway::Activity,
    gateway::Ready, guild::Guild as DiscordGuild, guild::Member, id::GuildId, id::UserId,
};
use serenity::prelude::{Context, EventHandler};
use std::collections::{HashMap, HashSet};

use crate::core::consts::DB as db;
use crate::db::models::{Guild, User};
use crate::{menu, monitors};

pub struct Zero2Handler {
    blacklist: HashSet<String>,
    guilds: HashMap<GuildId, Guild>,
    users: HashMap<UserId, User<Utc>>,
}

impl Default for Zero2Handler {
    fn default() -> Self {
        // TODO batch DB queries
        // https://docs.diesel.rs/diesel/pg/struct.TransactionBuilder.html
        let blacklist = HashSet::new();
        let guilds = db
            .all_guilds()
            .unwrap_or_else(|_| vec![])
            .into_iter()
            .map(|guild| (GuildId(guild.id as u64), guild))
            .collect();
        let users = db
            .all_users()
            .unwrap_or_else(|_| vec![])
            .into_iter()
            .map(|user| (UserId(user.id as u64), user))
            .collect();
        Zero2Handler {
            blacklist,
            guilds,
            users,
        }
    }
}

impl EventHandler for Zero2Handler {
    fn guild_create(&self, _ctx: Context, guild: DiscordGuild, _is_new: bool) {
        if !self.guilds.contains_key(&guild.id) {
            let _ = db.new_guild(guild.id);
        }
    }

    fn guild_member_addition(&self, context: Context, guild_id: GuildId, new_member: Member) {
        monitors::new_member_monitors(&context, guild_id, &new_member);

        // TODO should we be doing this? or should just let the user manually create a profile?
        // Insert new member to database
        if !self.users.contains_key(&new_member.user_id()) {
            let _ = db.new_user(
                new_member.user_id(),
                guild_id,
                new_member
                    .nick
                    .clone()
                    .unwrap_or_else(|| new_member.display_name().to_string()),
                new_member.roles,
            );
        }
    }

    fn reaction_add(&self, context: Context, add_reaction: Reaction) {
        menu::handle_reaction(&context, &add_reaction);
    }

    // TODO should we be doing this? or should just let the user manually create a profile?
    fn presence_update(&self, _context: Context, new_data: PresenceUpdateEvent) {
        if !self.users.contains_key(&new_data.presence.user_id) {
            let _ = db.new_user(
                new_data.presence.user_id,
                new_data.guild_id.unwrap_or(GuildId(0)),
                "".to_string(),
                new_data.roles.unwrap_or(vec![]),
            );
        }
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::listening("2!help"));

        for guild_id in ctx.cache.clone().read().all_guilds() {
            if let Ok(guild) = guild_id.to_partial_guild(&ctx) {
                info!("[GUILD] Available in {}", guild.name);
            }
        }

        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
