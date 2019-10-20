table! {
    blacklist (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        created_at -> Nullable<Timestamptz>,
        value -> Varchar,
    }
}

table! {
    commands_blacklist (id) {
        id -> Int4,
        name -> Varchar,
        blocked -> Bool,
    }
}

table! {
    guilds (id) {
        id -> Int8,
        admin_roles -> Array<Int8>,
        ignored_channels -> Array<Int8>,
        welcome -> Bool,
        welcome_channel -> Int8,
        welcome_message -> Text,
        prefix -> Text,
    }
}

table! {
    users (id, guild_id) {
        id -> Int8,
        guild_id -> Int8,
        username -> Text,
        roles -> Array<Int8>,
        access_level -> Int2,
        joined_at -> Nullable<Timestamptz>,
        left_at -> Nullable<Timestamptz>,
        messages_count -> Int4,
        anilist_id -> Nullable<Int4>,
        anilist_name -> Varchar,
        vip -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(blacklist, commands_blacklist, guilds, users,);
