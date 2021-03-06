-- Your SQL goes here-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  uname TEXT NOT NULL UNIQUE,
  password BYTEA NOT NULL,
  salt BYTEA NOT NULL
);

ALTER TABLE tasks ADD COLUMN user_id INT NOT NULL;

ALTER TABLE tasks
ADD CONSTRAINT user_id_fk FOREIGN KEY(user_id) REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE;