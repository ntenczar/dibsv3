-- Your SQL goes here
ALTER TABLE users
  ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT now();
