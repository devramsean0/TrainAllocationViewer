-- Remove duplicate rows before adding unique constraint (keeps oldest entry)
DELETE FROM allocations a USING allocations b
WHERE a.id > b.id
  AND a.origin_datetime = b.origin_datetime
  AND a.origin_location = b.origin_location
  AND a.dest_datetime = b.dest_datetime
  AND a.dest_location = b.dest_location
  AND a.resource_group_id = b.resource_group_id;

-- Add unique constraint for upsert support
CREATE UNIQUE INDEX allocations_unique_idx ON allocations (origin_datetime, origin_location, dest_datetime, dest_location, resource_group_id);
