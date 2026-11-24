CREATE TABLE posts (
    slug TEXT NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    published_at TEXT NOT NULL,
    updated_at TEXT,
    tags TEXT NOT NULL,
    reading_minutes INTEGER NOT NULL CHECK (reading_minutes > 0),
    featured INTEGER NOT NULL DEFAULT 0 CHECK (featured IN (0, 1)),
    content TEXT NOT NULL
);

CREATE INDEX posts_published_at_idx ON posts (published_at DESC);
