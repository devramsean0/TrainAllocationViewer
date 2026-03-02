-- Add migration script here
CREATE TABLE alloc_archive_log(
    id BIGSERIAL PRIMARY KEY,
    file_name TEXT NOT NULL UNIQUE
);