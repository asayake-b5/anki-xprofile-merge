-- Add migration script here
--
CREATE TABLE IF NOT EXISTS notes
(
    nid         INTEGER PRIMARY KEY NOT NULL,
    sentence    TEXT NOT NULL,
    image       TEXT,
    audio       TEXT,
    morphenes   TEXT
);
