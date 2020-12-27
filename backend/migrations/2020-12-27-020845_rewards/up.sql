-- Your SQL goes here

CREATE TABLE rewards (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  bspts INTEGER NOT NULL,
  CONSTRAINT user_id_fk FOREIGN KEY(user_id) REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE
)