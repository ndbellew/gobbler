CREATE TABLE app_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    delete_enabled INTEGER NOT NULL DEFAULT 0
);
INSERT INTO app_settings (id, delete_enabled) VALUES (1, 0);
