-- Add migration script here
ALTER TABLE schedules ALTER COLUMN headcode TYPE TEXT USING headcode::TEXT;
