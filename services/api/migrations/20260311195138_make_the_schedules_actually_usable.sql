-- Add migration script here
ALTER TABLE schedules ADD COLUMN origin_location TEXT NOT NULL;
ALTER TABLE schedules ADD COLUMN dest_location TEXT NOT NULL;
ALTER TABLE schedules ADD COLUMN start_date TEXT NOT NULL;
ALTER TABLE schedules ADD COLUMN end_date TEXT NOT NULL;

ALTER TABLE schedules DROP COLUMN allocation_id;
ALTER TABLE schedules DROP COLUMN date;

CREATE TABLE schedule_allocation(
    id BIGSERIAL PRIMARY KEY,
    schedule_id BIGSERIAL,
    allocation_id BIGSERIAL
);