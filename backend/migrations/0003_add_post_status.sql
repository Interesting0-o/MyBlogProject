ALTER TABLE posts ADD COLUMN status TEXT NOT NULL DEFAULT 'published' CHECK (status IN ('draft', 'published'));

CREATE INDEX posts_status_idx ON posts (status);
