-- This file should undo anything in `up.sql`

ALTER TABLE rewards DROP COLUMN icon;
ALTER TABLE tasks DROP COLUMN icon;