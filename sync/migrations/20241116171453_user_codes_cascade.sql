ALTER TABLE codes RENAME TO codes_old;

CREATE TABLE IF NOT EXISTS codes (
  id TEXT PRIMARY KEY NOT NULL,
  owner_id TEXT NOT NULL,
  display_name TEXT NOT NULL,
  icon_url TEXT,
  website_url TEXT,
  FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);

INSERT INTO codes (id, display_name, icon_url, website_url, owner_id)
SELECT id, display_name, icon_url, website_url, owner_id
FROM codes_old;

DROP TABLE codes_old;
