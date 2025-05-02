CREATE TABLE IF NOT EXISTS Recipe {
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    instructions TEXT NOT NULL,
    summary VARCHAR(255) NOT NULL,
    ingredients TEXT NOT NULL,
    views INTEGER DEFAULT 0 NOT NULL,
    ratings INTEGER DEFAULT 0 NOT NULL,
    total_rating INTEGER DEFAULT 0 NOT NULL,
    -- The URL where the recipe came from. May not have a source
    source TEXT
}