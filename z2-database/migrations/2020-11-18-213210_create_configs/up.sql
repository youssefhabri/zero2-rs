-- configs table
CREATE TABLE configs (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    value TEXT NOT NULL,
    guild_id BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT ux_name_guildid UNIQUE (name, guild_id)
);

CREATE UNIQUE INDEX guild_config_entry_idx
ON configs (name, guild_id)
WHERE guild_id IS NOT NULL;
