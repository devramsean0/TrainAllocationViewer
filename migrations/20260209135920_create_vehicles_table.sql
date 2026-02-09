-- Add migration script here
CREATE TABLE vehicles(
    id INT PRIMARY KEY,
    livery TEXT NOT NULL,
    decor TEXT NOT NULL,
    vehicle_type TEXT NOT NULL,
    specific_type TEXT NOT NULL
)