-- Add migration script here
CREATE TABLE locations(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nlc TEXT NOT NULL UNIQUE,
    stanox TEXT,
    tiploc TEXT,
    crs TEXT,
    uic TEXT,
    nlcdesc TEXT,
    axis TEXT,
    nlcdesc16 TEXT
)