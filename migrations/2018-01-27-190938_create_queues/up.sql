-- Your SQL goes here
CREATE TABLE queues (
  id VARCHAR(80) PRIMARY KEY,
  title VARCHAR NOT NULL,
  frozen BOOLEAN NOT NULL DEFAULT 'f'
)
