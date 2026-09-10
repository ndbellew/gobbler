import { useState } from "react";
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
}

function App() {
  const [files, setFiles] = useState<FileEntry[]>([]);
  const [status, setStatus] = useState("Waiting");
  const [folderName, setFolderName] = useState("");
  const [selectedFolder, setSelectedFolder] = useState<string | null>(null);
  const [stageAfterDays, setStageAfterDays] = useState(2);
  const [deleteAfterDays, setDeleteAfterDays] = useState(7);
  

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

  const sendWatchRule = async () => {
    const ruleIsValid = deleteAfterDays > stageAfterDays;
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
          Delete after
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