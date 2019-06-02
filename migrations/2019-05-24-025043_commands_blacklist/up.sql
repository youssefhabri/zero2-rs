-- Your SQL goes here
CREATE TABLE commands_blacklist
(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    blocked BOOLEAN NOT NULL DEFAULT 'f'
)