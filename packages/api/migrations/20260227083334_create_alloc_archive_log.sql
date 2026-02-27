-- Add migration script here
CREATE TABLE alloc_archive_log(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_name TEXT NOT NULL UNIQUE
);