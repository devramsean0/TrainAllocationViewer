-- Add migration script here
ALTER TABLE allocations ADD COLUMN sequence_number INTEGER;
ALTER TABLE allocations ADD COLUMN resource_group_position INTEGER;
ALTER TABLE allocations ADD COLUMN diagram_no TEXT;
ALTER TABLE allocations ADD COLUMN origin_miles INTEGER;
ALTER TABLE allocations ADD COLUMN destination_miles INTEGER;
ALTER TABLE allocations ADD COLUMN reversed TEXT;
ALTER TABLE allocations ADD COLUMN origin_country_code_iso TEXT;
ALTER TABLE allocations ADD COLUMN origin_subsidiary_information_code TEXT;
ALTER TABLE allocations ADD COLUMN origin_subsidiary_information_company TEXT;
ALTER TABLE allocations ADD COLUMN dest_country_code_iso TEXT;
ALTER TABLE allocations ADD COLUMN dest_subsidiary_information_code TEXT;
ALTER TABLE allocations ADD COLUMN dest_subsidiary_information_company TEXT;
ALTER TABLE allocations ADD COLUMN allocation_origin_country_code_iso TEXT;
ALTER TABLE allocations ADD COLUMN allocation_origin_subsidiary_information_code TEXT;
ALTER TABLE allocations ADD COLUMN allocation_origin_subsidiary_information_company TEXT;
ALTER TABLE allocations ADD COLUMN allocation_dest_country_code_iso TEXT;
ALTER TABLE allocations ADD COLUMN allocation_dest_subsidiary_information_code TEXT;
ALTER TABLE allocations ADD COLUMN allocation_dest_subsidiary_information_company TEXT;

ALTER TABLE resource_groups ADD COLUMN resource_type TEXT;
ALTER TABLE resource_groups ADD COLUMN status TEXT;
ALTER TABLE resource_groups ADD COLUMN end_of_day_miles TEXT;

ALTER TABLE vehicles ADD COLUMN resource_position INT;
ALTER TABLE vehicles ADD COLUMN planned_resource_group TEXT;
ALTER TABLE vehicles ADD COLUMN length_value TEXT;
ALTER TABLE vehicles ADD COLUMN length_measure TEXT;
ALTER TABLE vehicles ADD COLUMN weight INT;
ALTER TABLE vehicles ADD COLUMN special_characteristics TEXT;
ALTER TABLE vehicles ADD COLUMN seat_count INT;
ALTER TABLE vehicles ADD COLUMN cab_count INT;
ALTER TABLE vehicles ADD COLUMN date_entered_service TEXT;
ALTER TABLE vehicles ADD COLUMN date_registered TEXT;
ALTER TABLE vehicles ADD COLUMN category TEXT;
ALTER TABLE vehicles ADD COLUMN brake_type TEXT;
ALTER TABLE vehicles ADD COLUMN max_speed TEXT;




