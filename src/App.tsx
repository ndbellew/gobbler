import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

type FileEntry = {
  name: string;
  path: string;
  size: number;
};

function App() {
  const [files, setFiles] = useState<FileEntry[]>([]);
  const [status, setStatus] = useState("Waiting");
  const [selectedFolder, setSelectedFolder] = useState<string | null>(null);


  const chooseFolder = async () => {
    const folder = await open({
      directory: true,
      multiple: false,
      title: "Choose a folder for Gobbler",
    });

    if (folder) {
      setSelectedFolder(folder);
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

      <p>{status}</p>

      <ul>
        {files.map((file) => (
          <li key={file.path}>
            {file.name} - {file.size} bytes
          </li>
        ))}
      </ul>
    </main>
  );
}

export default App;