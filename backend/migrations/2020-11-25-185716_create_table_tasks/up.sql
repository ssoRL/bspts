-- Your SQL goes here

CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  bspts INTEGER NOT NULL,
  is_done BOOLEAN NOT NULL DEFAULT 'f',
  next_reset DATE NOT NULL,
  every INTEGER NOT NULL,
  time_unit TEXT NOT NULL,
  by_when INTEGER NOT NULL
)