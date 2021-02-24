-- Your SQL goes here
CREATE TABLE users (
    -- id SERIAL NOT NULL PRIMARY KEY,
    id UUID PRIMARY KEY,
    username TEXT NOT NULL,
    passwd TEXT NOT NULL
);
