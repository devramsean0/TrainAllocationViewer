-- Add migration script here
CREATE TABLE allocations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    origin_datetime TEXT NOT NULL,
    origin_location TEXT NOT NULL,
    date TEXT,
    dest_location TEXT NOT NULL,
    dest_datetime TEXT NOT NULL,
    allocation_origin_datetime TEXT NOT NULL,
    allocation_origin_location TEXT NOT NULL,
    allocation_dest_datetime TEXT NOT NULL,
    allocation_dest_location TEXT NOT NULL
);