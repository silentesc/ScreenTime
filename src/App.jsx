import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const saveData = () => {
    invoke("save_data");
  };

  return (
    <div className="content">
      <button onClick={saveData}>Save data</button>
    </div>
  );
}

export default App;
