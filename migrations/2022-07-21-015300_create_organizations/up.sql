-- Your SQL goes here
CREATE TABLE organizations (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  active BOOLEAN DEFAULT 'f',
  logo VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON organizations
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();