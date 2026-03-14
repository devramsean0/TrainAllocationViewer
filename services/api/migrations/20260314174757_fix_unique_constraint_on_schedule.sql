-- Add migration script here

DROP INDEX IF EXISTS schedules_unique_idx;
CREATE UNIQUE INDEX schedules_unique_idx ON schedules (uid, identity, headcode, start_date, end_date, atoc_code);