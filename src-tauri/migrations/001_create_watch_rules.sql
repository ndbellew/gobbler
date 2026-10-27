CREATE TABLE watch_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    stage_after_days INTEGER NOT NULL,
    delete_after_days INTEGER NOT NULL
);