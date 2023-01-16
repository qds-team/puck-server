-- Add migration script here
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS manga (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL,
    path        TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS setting (
    id          INTEGER PRIMARY KEY NOT NULL,
    key         TEXT                NOT NULL,
    value       TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS manga_files (
    id          INTEGER PRIMARY KEY NOT NULL,
    manga_id    INTEGER             NOT NULL,
    filename    TEXT                NOT NULL
);

ALTER TABLE setting 
ADD password TEXT