-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  lastname VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  credential_id SERIAL REFERENCES credentials(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();