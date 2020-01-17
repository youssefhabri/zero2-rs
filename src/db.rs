use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use std::collections::HashSet;
use std::ops::Deref;

pub mod models;
mod schema;

use self::models::*;
use self::schema::*;
use chrono::{DateTime, Utc};
use diesel::result::Error;
use serenity::model::id::{GuildId, RoleId, UserId};

pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn connect() -> Self {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(15)
            .build(manager)
            .expect("Failed to create connection pool");

        Database { pool }
    }

    fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool
            .clone()
            .get()
            .expect("Attempt to get connection timed out")
    }

    pub fn blacklist(&self, type_: &str) -> QueryResult<HashSet<String>> {
        let blacklist = blacklist::table
            .load::<(i32, String, Option<DateTime<Utc>>, String)>(self.conn().deref())?
            .iter()
            .filter(|(_, b_type, _, _)| b_type == type_)
            .map(|(_, _, _, value)| value.clone())
            .collect::<HashSet<String>>();

        Ok(blacklist)
    }

    pub fn new_guild(&self, id: GuildId) -> QueryResult<Guild> {
        let id = *id.as_u64() as i64;
        let guild = NewGuild { id };
        diesel::insert_into(guilds::table)
            .values(&guild)
            .on_conflict_do_nothing()
            .get_result(self.conn().deref())
    }

    pub fn new_user(
        &self,
        id: UserId,
        guild_id: GuildId,
        username: String,
        roles: Vec<RoleId>,
    ) -> QueryResult<User<Utc>> {
        let id = *id.as_u64() as i64;
        let guild_id = *guild_id.as_u64() as i64;
        let roles = roles.iter().map(|role| *role.as_u64() as i64).collect();
        let user = NewUser {
            id,
            guild_id,
            username,
            roles,
        };
        diesel::insert_into(users::table)
            .values(&user)
            .on_conflict_do_nothing()
            .get_result(self.conn().deref())
    }

    pub fn update_anilist_name(
        &self,
        id: i64,
        anilist_id: i32,
        anilist_name: String,
    ) -> QueryResult<User<Utc>> {
        let user = AnilistNameUpdate {
            id,
            anilist_id,
            anilist_name,
        };
        diesel::update(users::table)
            .set(&user)
            .get_result(self.conn().deref())
    }

    pub fn find_user(&self, id: UserId) -> QueryResult<User<Utc>> {
        let id = *id.as_u64() as i64;
        let mut res = users::table
            .load::<User<Utc>>(self.conn().deref())?
            .into_iter()
            .filter(|user| user.id == id)
            .collect::<Vec<User<Utc>>>();

        // TODO can this be improved?
        if !res.is_empty() {
            return Ok(res.remove(0));
        }

        Err(Error::NotFound)
    }

    pub fn delete_user(&self, id: UserId, guild_id: GuildId) -> QueryResult<usize> {
        diesel::delete(users::table.find((*id.as_u64() as i64, *guild_id.as_u64() as i64)))
            .execute(self.conn().deref())
    }

    pub fn all_users(&self) -> QueryResult<Vec<User<Utc>>> {
        users::table.load::<User<Utc>>(self.conn().deref())
    }

    pub fn all_guilds(&self) -> QueryResult<Vec<Guild>> {
        guilds::table.load::<Guild>(self.conn().deref())
    }

    pub fn find_guild(&self, guild_id: GuildId) -> QueryResult<Guild> {
        let guild_id = *guild_id.as_u64() as i64;
        let mut res = guilds::table
            .load::<Guild>(self.conn().deref())?
            .into_iter()
            .filter(|guild| guild.id == guild_id)
            .collect::<Vec<Guild>>();

        if !res.is_empty() {
            return Ok(res.remove(0));
        }

        Err(Error::NotFound)
    }

    pub fn find_command(&self, command: String) -> QueryResult<CustomCommand> {
        let mut res = custom_commands::table
            .load::<CustomCommand>(self.conn().deref())?
            .into_iter()
            .filter(|cmd| cmd.name == command)
            .collect::<Vec<CustomCommand>>();

        if !res.is_empty() {
            return Ok(res.remove(0));
        }

        Err(Error::NotFound)
    }
}
