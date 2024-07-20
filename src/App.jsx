import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {

  return (
    <div className="content">
      <h1>ScreeTime</h1>
      <button
        onClick={async () => {
          const response = await invoke("get_screen_time_apps", { date: "20.07.2024", sortMode: "millis_in_foreground", reversed: true });
          for (const app of response) {
            console.log(app.display_name, app.millis_in_foreground["20.07.2024"]);
          }
        }}>
        Get Screen Time
      </button>
    </div>
  );
}

export default App;
