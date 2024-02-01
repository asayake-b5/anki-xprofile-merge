-- Add migration script here
--
CREATE TABLE IF NOT EXISTS knownmorphs
(
    lemma                     TEXT NOT NULL,
    inflection                TEXT NOT NULL,
    highest_learning_interval INTEGER NOT NULL,
    primary key (lemma, inflection)
);
