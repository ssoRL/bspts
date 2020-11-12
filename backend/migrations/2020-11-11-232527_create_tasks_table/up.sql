-- Your SQL goes here

CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  bspts INTEGER,
  is_done BOOLEAN NOT NULL DEFAULT 'f',
  next_reset DATE,
  frequency INTERVAL
)