-- This file should undo anything in `up.sql`

ALTER TABLE queues DROP COLUMN created_at;
