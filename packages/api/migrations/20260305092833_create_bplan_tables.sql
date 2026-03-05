-- Add migration script here
CREATE TABLE bplan_log(
    id BIGSERIAL PRIMARY KEY,
    file_version TEXT NOT NULL,
    source_system TEXT NOT NULL,
    toc_id TEXT NOT NULL,
    timetable_start_date TEXT NOT NULL,
    timetable_end_date TEXT NOT NULL,
    cycle_type TEXT NOT NULL,
    cycle_stage TEXT NOT NULL,
    creation_date TEXT UNIQUE NOT NULL,
    sequence_number SERIAL NOT NULL
);

CREATE TABLE reference_codes(
    action_code TEXT NOT NULL,
    code_type TEXT NOT NULL,
    code TEXT,
    description TEXT NOT NULL UNIQUE
);