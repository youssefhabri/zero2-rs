use super::schema::*;
use chrono::{DateTime, TimeZone};

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[table_name = "guilds"]
#[primary_key(id)]
pub struct Guild {
    pub id: i64,
    pub admin_roles: Vec<i64>,
    pub ignored_channels: Vec<i64>,
    pub welcome: bool,
    pub welcome_channel: i64,
    pub welcome_message: String,
    pub prefix: String, // TODO implement a per-guild prefix?
}

#[derive(Insertable)]
#[table_name = "guilds"]
pub struct NewGuild {
    pub id: i64,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[primary_key(id, guild_id)]
pub struct User<Tz: TimeZone> {
    pub id: i64,
    pub guild_id: i64,
    pub username: String,
    pub roles: Vec<i64>,
    pub access_level: i16,
    pub joined_at: Option<DateTime<Tz>>,
    pub left_at: Option<DateTime<Tz>>,
    pub messages_count: i32,
    pub anilist_id: Option<i32>,
    pub anilist_name: String,
    pub vip: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub id: i64,
    pub guild_id: i64,
    pub username: String,
    pub roles: Vec<i64>,
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "users"]
#[primary_key(id, guild_id)]
pub struct AnilistNameUpdate {
    pub id: i64,
    pub anilist_id: i32,
    pub anilist_name: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
pub struct CustomCommand {
    pub id: i64,
    pub guild_id: i64,
    pub name: String,
    pub kind: String, // 'text', 'simple_parsable'
    pub content: String,
}
