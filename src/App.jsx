import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {

  return (
    <div className="content">
      <h1>ScreeTime</h1>
      <button
        onClick={async () => {
          const date = await invoke("get_today_date");

          const sorted_apps = await invoke("get_screen_time_apps_sorted", { date: date, sortMode: "millis_in_foreground", reversed: true });
          console.log("millis_in_foreground millis_in_foreground millis_in_foreground");
          for (const app of sorted_apps) {
            console.log(app.display_name, app.millis_in_foreground[date]);
          }
        }}>
        Get Screen Time Foreground
      </button> <br /> <br />

      <button
        onClick={async () => {
          const date = await invoke("get_today_date");

          const sorted_apps = await invoke("get_screen_time_apps_sorted", { date: date, sortMode: "millis_in_background", reversed: true });
          console.log("millis_in_background millis_in_background millis_in_background");
          for (const app of sorted_apps) {
            console.log(app.display_name, app.millis_in_background[date]);
          }
        }}>
        Get Screen Time Background
      </button> <br /> <br />

      <button
        onClick={async () => {
          const date = await invoke("get_today_date");

          const sorted_apps = await invoke("get_screen_time_apps_sorted", { date: date, sortMode: "times_opened", reversed: true });
          console.log("times_opened times_opened times_opened");
          for (const app of sorted_apps) {
            console.log(app.display_name, app.times_opened[date]);
          }
        }}>
        Get Screen Time Times Opened
      </button> <br /> <br />

      <button
        onClick={async () => {
          const date = await invoke("get_today_date");

          const sorted_apps = await invoke("get_screen_time_apps_sorted", { date: date, sortMode: "times_focused", reversed: true });
          console.log("times_focued times_focued times_focued");
          for (const app of sorted_apps) {
            console.log(app.display_name, app.times_focused[date]);
          }
        }}>
        Get Screen Time Times Focused
      </button> <br /> <br />

      <button
        onClick={async () => {
          const date = await invoke("get_today_date");

          const app = await invoke("get_screen_time_app_by_name", { appName: "Discord.exe", ignoreCase: true });
          console.log("get_screen_time_app_by_name get_screen_time_app_by_name get_screen_time_app_by_name");
          console.log(app);
        }}>
        Get Screen Time App By Name (Discord.exe)
      </button> <br /> <br />
    </div>
  );
}

export default App;
