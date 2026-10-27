CREATE UNIQUE INDEX idx_tracked_files_rule_path
ON tracked_files(watch_rule_id, path);