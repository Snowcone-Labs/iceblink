CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY NOT NULL,
  username TEXT NOT NULL,
  display_name TEXT NOT NULL,
  avatar_url TEXT NOT NULL,
  upstream_userid TEXT NOT NULl
);

CREATE TABLE IF NOT EXISTS codes (
  id TEXT PRIMARY KEY NOT NULL,
  owner_id TEXT NOT NULL,
  display_name TEXT NOT NULL,
  content TEXT NOT NULL,
  icon_url TEXT,
  website_url TEXT,
  FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);
