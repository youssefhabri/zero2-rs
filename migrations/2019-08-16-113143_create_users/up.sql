-- Users table
CREATE TABLE users
(
    id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    username TEXT NOT NULL DEFAULT '',
    roles BIGINT [] NOT NULL  DEFAULT array[]::bigint[],
    access_level SMALLINT NOT NULL DEFAULT 1,
    joined_at TIMESTAMP WITH TIME ZONE,
    left_at TIMESTAMP WITH TIME ZONE,
    messages_count INT NOT NULL DEFAULT 0,
    anilist_id INT NULL,
    anilist_name VARCHAR NOT NULL DEFAULT '',
    vip BOOLEAN NOT NULL DEFAULT 'f',
    PRIMARY KEY (id, guild_id)
)