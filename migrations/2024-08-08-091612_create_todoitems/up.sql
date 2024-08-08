-- Your SQL goes here
CREATE TABLE todo_items (
  id UUID PRIMARY KEY,
  title TEXT NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT FALSE,
  create_at TIMESTAMPTZ NOT NULL
)