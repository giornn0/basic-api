-- Your SQL goes here
CREATE TABLE credentials (
  id SERIAL PRIMARY KEY,
  password TEXT NOT NULL,
  email VARCHAR NOT NULL,
  state BOOLEAN DEFAULT 'f',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON credentials
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();