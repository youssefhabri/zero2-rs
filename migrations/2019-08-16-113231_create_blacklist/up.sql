-- Commands blacklist
CREATE TABLE blacklist
(
    id SERIAL PRIMARY KEY,
    type VARCHAR NOT NULL DEFAULT 'user',
    created_at TIMESTAMP WITH TIME ZONE,
    value VARCHAR NOT NULL
)