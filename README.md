# Gobbler

Gobbler is a Tauri desktop app that tracks regular files in configured folders. The Rust process owns the SQLite state and runs reconciliation at startup and once per minute while the app is running. It continues if the window is hidden, but **not after the app exits or the user logs out**; per-OS login startup/service installation is future work.

## File lifecycle

- Age is measured from first observation by Gobbler, not filesystem creation time. A size or modification-time change resets first-seen and clears the staged mark.
- Files are marked after `stage_after_days`; changing a rule changes future thresholds but does not reset existing timers.
- `delete_after_days` is a grace period counted from `staged_at`. Cancel Mark clears the mark and restarts the age timer.
- Renamed/missing paths are kept as `Missing`; a reappearing file starts a fresh timer.
- Deletion is **dry-run by default** (`app_settings.delete_enabled = 0`). Eligible files are logged, never removed. Do not enable real deletion until manual verification and recovery behavior are agreed.
- The scanner is non-recursive and considers regular files directly in the selected folder.

## Development

Use `npm run tauri dev` to run the desktop application. Build/package support and background startup behavior still need to be verified separately on each target OS. The current worker is app-lifetime only, not an installed system service.
