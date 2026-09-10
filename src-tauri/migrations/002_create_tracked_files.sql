CREATE TABLE tracked_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    watch_rule_id INTEGER NOT NULL,
    path TEXT NOT NULL,
    first_seen_at INTEGER NOT NULL,
    staged_at INTEGER,
    deleted_at INTEGER,
    status TEXT NOT NULL,
    size INTEGER NOT NULL,
    FOREIGN KEY (watch_rule_id)
        REFERENCES watch_rules(id)
);