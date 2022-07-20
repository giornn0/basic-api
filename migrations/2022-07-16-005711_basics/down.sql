-- This file should undo anything in `up.sql`
DROP FUNCTION IF EXISTS trigger_set_updated_at();
DROP TABLE IF EXISTS tokens;