-- Your SQL goes here
CREATE TABLE custom_commands (
    id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    name VARCHAR NOT NULL,
    kind VARCHAR NOT NULL DEFAULT 'text',
    content TEXT NOT NULL,
    PRIMARY KEY (id, guild_id)
);

-- pub struct CustomCommand {
--     pub id: i64,
--     pub guild_id: i64,
--     pub name: String,
--     pub kind: CommandKind,
--     pub content: String,
-- }