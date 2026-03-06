-- Add migration script here
CREATE table cif_schedule_log(
    id BIGSERIAL PRIMARY KEY,
    mainframe_identity TEXT NOT NULL,
    extract_date TEXT NOT NULL,
    extract_time TEXT NOT NULL,
    file_reference TEXT NOT NULL,
    version CHAR NOT NULL
)

CREATE TABLE schedule(
    id BIGSERIAL PRIMARY KEY,
    uid TEXT NOT NULL,
    identity TEXT NOT NULL,
    headcode INT NULL,
    date TEXT NOT NULL,
    allocation_id BIGSERIAL,
    indicator TEXT NOT NULL,
    atoc_code TEXT NOT NULL,
    performance_monitoring BOOLEAN NOT NULL
)

CREATE TABLE schedule_location(
    id BIGSERIAL PRIMARY KEY,
    location TEXT NOT NULL,
    scheduled_departure_time TEXT,
    scheduled_arrival_time TEXT,
    scheduled_pass_time TEXT,
    public_departure_time TEXT,
    public_arrival_time TEXT,
    platform TEXT,
    line TEXT,
    engineering_allowance TEXT,
    pathing_allowance TEXT,
    performance_allowance TEXT,
    activity TEXT,
    schedule_id BIGSERIAL NOT NULL
)