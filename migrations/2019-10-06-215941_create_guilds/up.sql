-- Guilds table
CREATE TABLE guilds
(
    id BIGINT NOT NULL,
    admin_roles BIGINT [] NOT NULL DEFAULT array[]::bigint[],
    ignored_channels BIGINT [] NOT NULL DEFAULT array[]::bigint[],
    welcome BOOL NOT NULL DEFAULT 'f',
    welcome_channel BIGINT NOT NULL DEFAULT 0,
    welcome_message TEXT NOT NULL DEFAULT '',
    prefix TEXT NOT NULL DEFAULT '2!',
    PRIMARY KEY (id) 
)
