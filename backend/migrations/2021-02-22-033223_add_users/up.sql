-- Your SQL goes here
CREATE TABLE users (
    -- id SERIAL NOT NULL PRIMARY KEY,
    id UUID PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL
);
