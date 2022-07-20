-- Your SQL goes here

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE tokens (
  id SERIAL PRIMARY KEY,
  model VARCHAR NOT NULL,
  token TEXT NOT NULL,
  expiration_date TIMESTAMPTZ ,
  state BOOLEAN DEFAULT 'f',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);