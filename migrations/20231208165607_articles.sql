-- Add migration script here
CREATE TABLE IF NOT EXISTS articles (
  id UUID PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  auther_id UUID NOT NULL,
  body TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  UNIQUE(title)
);
