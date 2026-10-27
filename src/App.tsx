import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

type FileEntry = {
  name: string;
  path: string;
  size: number;
};

type WatchRuleEntry = {
  path: string;
  stageAfterDays: number;
  deleteAfterDays: number;
};

type TrackedFile = { id: number; path: string; status: string; ageDays: number; stagedAt: number | null; deleteAt: number | null; size: number };

function App() {
  const [files, setFiles] = useState<FileEntry[]>([]);
  const [trackedFiles, setTrackedFiles] = useState<TrackedFile[]>([]);
  const [status, setStatus] = useState("Waiting");
  const [folderName, setFolderName] = useState("");
  const [selectedFolder, setSelectedFolder] = useState<string | null>(null);
  const [stageAfterDays, setStageAfterDays] = useState(2);
  const [deleteAfterDays, setDeleteAfterDays] = useState(7);

  useEffect(() => {
    invoke<WatchRuleEntry[]>("get_watch_rules").then((rules) => {
      if (rules.length > 0) {
        const rule = rules[0];
        setSelectedFolder(rule.path); setFolderName(rule.path);
        setStageAfterDays(rule.stageAfterDays); setDeleteAfterDays(rule.deleteAfterDays);
      }
    }).catch((error) => setStatus("ERROR: " + String(error)));
    invoke<TrackedFile[]>("get_tracked_files").then(setTrackedFiles).catch(() => {});
  }, []);
  

  const chooseFolder = async () => {
    const folder = await open({
      directory: true,
      multiple: false,
      title: "Choose a folder for Gobbler",
    });

    if (folder) {
      setSelectedFolder(folder);
      setFolderName(folder);
    }
  };

  const refreshTrackedFiles = async () => {
    try { setTrackedFiles(await invoke<TrackedFile[]>("get_tracked_files")); }
    catch (error) { setStatus(`ERROR: ${String(error)}`); }
  };

  const cancelMark = async (path: string) => {
    try { await invoke("cancel_file_mark", { path }); await refreshTrackedFiles(); }
    catch (error) { setStatus("ERROR: " + String(error)); }
  };

  const sendWatchRule = async () => {
    const ruleIsValid = stageAfterDays >= 0 && deleteAfterDays > 0;
    if (ruleIsValid && selectedFolder) {
      setStatus("Rule is Valid, Saving..");

      try {
        const result = await invoke<WatchRuleEntry>("save_watch_rule", {
          path: selectedFolder,
          stageAfterDays,
          deleteAfterDays,
        });

        setStatus("Rule Saved");
        console.log(result);
        await refreshTrackedFiles();
      } catch (error) {
        setStatus(`ERROR: ${String(error)}`);
      }
    } else if (!ruleIsValid && selectedFolder){
      setStatus(`Rule is not Valid`);
    } else if (ruleIsValid && !selectedFolder) {
      setStatus("Folder not Selected!")
    } else {
      setStatus("Folder not Selected and Rule is not Valid!!")
    }
  };

  const scanFolder = async () => {
    setStatus("Scanning...");
    if (selectedFolder) {
      try {
        const result = await invoke<FileEntry[]>("scan_folder", {
          path: selectedFolder,
        });

        setFiles(result);
        await refreshTrackedFiles();
        setStatus(`Found ${result.length} files`);
      } catch (error) {
        setStatus(`ERROR: ${String(error)}`);
      }
    }
    else {
      setStatus("No Folder Selected")
    }
  };

  return (
    <main>
      <h1>Gobbler</h1>

      <button onClick={chooseFolder}>Choose Folder</button>
      <button onClick={scanFolder}>Scan</button>
      <div>
        <label>
          Stage after
          <input
            type="number"
            min="0"
            value={stageAfterDays}
            onChange={(event) => setStageAfterDays(Number(event.target.value))}
          />
          days
        </label>
      </div>

      <div>
        <label>
          Delete after staging
          <input
            type="number"
            min="1"
            value={deleteAfterDays}
            onChange={(event) => setDeleteAfterDays(Number(event.target.value))}
          />
          days
        </label>
      </div>
      <button onClick={sendWatchRule}>
        Save Rule
      </button>


      <p>{folderName}</p>
      <p>{status}</p>


      <p><strong>Dry-run mode:</strong> eligible files are reported but never deleted.</p>
      <h2>Tracked files</h2>
      <button onClick={refreshTrackedFiles}>Refresh status</button>
      <ul>
        {trackedFiles.map((file) => (
          <li key={file.path}>
            {file.path} — {file.status}, {file.ageDays} days old, {file.size} bytes
            {file.deleteAt ? ` — scheduled for ${new Date(file.deleteAt * 1000).toLocaleString()}` : ""}
            {file.status === "Staged" && <button onClick={() => cancelMark(file.path)}>Cancel mark</button>}
          </li>
        ))}
      </ul>

      <h2>Latest scan</h2>
      <ul>
        {files.map((file) => (
          <li key={file.path}>
            {file.name} - {file.size} bytes
          </li>
        ))}
      </ul>
    </main >
  );
}

export default App;