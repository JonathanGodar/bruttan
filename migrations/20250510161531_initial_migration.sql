-- Add migration script here
-- PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS item (
    id INTEGER PRIMARY KEY NOT NULL,
    description TEXT
);


CREATE TABLE IF NOT EXISTS item_group (
    id INTEGER PRIMARY KEY NOT NULL,
    description TEXT
);


CREATE TABLE item_grouping (
    item INTEGER NOT NULL,
    item_group INTEGER NOT NULL,
    PRIMARY KEY (item, item_group),
    FOREIGN KEY (item) REFERENCES item(id) ON DELETE CASCADE,
    FOREIGN KEY (item_group) REFERENCES item_group(id) ON DELETE CASCADE
);

