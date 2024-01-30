-- Add migration script here
--
CREATE TABLE IF NOT EXISTS decks
(
    id         INTEGER PRIMARY KEY NOT NULL,
    timestamp  DATETIME DEFAULT CURRENT_TIMESTAMP
);
