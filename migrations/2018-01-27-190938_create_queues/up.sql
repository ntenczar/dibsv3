-- Your SQL goes here
CREATE TABLE queues (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  frozen BOOLEAN NOT NULL DEFAULT 'f'
)
