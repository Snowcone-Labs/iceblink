CREATE TABLE IF NOT EXISTS codes (
  id TEXT PRIMARY KEY NOT NULL,
  owner_id TEXT NOT NULL,
  display_name TEXT NOT NULL,
  icon_url TEXT,
  website_url TEXT
)
