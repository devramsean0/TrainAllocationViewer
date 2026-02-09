-- Add migration script here
CREATE TABLE resource_groups(
    id TEXT PRIMARY KEY,
    fleet TEXT NOT NULL
);

ALTER TABLE allocations ADD COLUMN resource_group_id TEXT NOT NULL;
ALTER TABLE vehicles ADD COLUMN resource_group_id TEXT NOT NULL;