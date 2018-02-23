-- Your SQL goes here
ALTER TABLE queues
  ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT now();
