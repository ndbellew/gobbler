CREATE TABLE deleted_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_name TEXT NOT NULL,
    path TEXT NOT NULL,
    size INTEGER NOT NULL,
    deleted_at INTEGER NOT NULL
);