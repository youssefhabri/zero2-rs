table! {
    configs (id) {
        id -> Int4,
        name -> Varchar,
        value -> Text,
        guild_id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
