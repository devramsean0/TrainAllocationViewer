-- Add migration script here
DELETE FROM locations;

ALTER TABLE locations ADD COLUMN display TEXT NOT NULL;
ALTER TABLE locations ADD COLUMN display_type TEXT NOT NULL;