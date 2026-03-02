-- Add migration script here
CREATE TABLE vehicles(
    id BIGINT PRIMARY KEY,
    livery TEXT NOT NULL,
    decor TEXT,
    vehicle_type TEXT NOT NULL,
    specific_type TEXT NOT NULL
)