<template>
    <div class="apps">
        <div class="app" v-for="app in apps" :key="app.display_name" @click="app.name">
            <div>{{ app.display_name }}</div>
            <div class="time">
                <div class="time-bar" :style="{
                    width: `${calculatePercentage(
                        app[sortMode]
                    )}%`,
                }"></div>
                {{ app.display_value }}
            </div>
        </div>
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api';
import { onMounted, ref, watch } from 'vue';

export default {
    props: {
        sortMode: String,
    },
    setup(props) {
        const apps = ref([]);

        const getApps = async () => {
            apps.value = [];

            const date = await invoke("get_today_date");
            const sortedApps = await invoke("get_screen_time_apps_sorted", { date: date, sortMode: props.sortMode, reversed: true });

            for (let app of sortedApps) {
                switch (props.sortMode) {
                    case "millis_in_foreground":
                        apps.value.push({
                            name: app.name,
                            display_name: app.display_name,
                            millis_in_foreground: app.millis_in_foreground[date],
                            display_value: calculateDisplayValue(app.millis_in_foreground[date]),
                        });
                        break;
                    case "millis_in_background":
                        apps.value.push({
                            name: app.name,
                            display_name: app.display_name,
                            millis_in_background: app.millis_in_background[date],
                            display_value: calculateDisplayValue(app.millis_in_background[date]),
                        });
                        break;
                    case "times_opened":
                        apps.value.push({
                            name: app.name,
                            display_name: app.display_name,
                            times_opened: app.times_opened[date],
                            display_value: app.times_opened[date],
                        });
                        break;
                    case "times_focused":
                        apps.value.push({
                            name: app.name,
                            display_name: app.display_name,
                            times_focused: app.times_focused[date],
                            display_value: app.times_focused[date],
                        });
                        break;
                }
            }
        };

        const calculatePercentage = (value) => {
            const maxValue = Math.max(...apps.value.map(app => app[props.sortMode]));
            const percentage = (value / maxValue) * 100;
            return percentage;
        };

        const calculateDisplayValue = (value) => {
            value = value / 1000;
            if (value < 60) {
                return `${Math.floor(value)}s`;
            }
            else if (value < 3600) {
                return `${Math.floor(value / 60)}m ${Math.floor(value % 60)}s`;
            }
            else {
                return `${Math.floor(value / 3600)}h ${Math.floor((value % 3600) / 60)}m`;
            }
        };

        const openAppDetails = (appName) => {
            console.log(appName);
        };

        watch(() => props.sortMode, () => {
            getApps();
        });

        onMounted(() => {
            getApps();
        });

        return { apps, getApps, calculatePercentage, openAppDetails };
    }
};
</script>

<style>
.apps {
    display: grid;
    grid-template-columns: repeat(1, 1fr);
    gap: 1rem;
    width: 75%;
}

.app {
    padding: 1rem 2rem;
    background: #838383;
    border: 2px solid rgb(204, 204, 204);
    border-radius: 0.5rem;
    cursor: pointer;
}

.app:hover {
    background: #9c9c9c;
}

.app div {
    white-space: nowrap;
}

.time {
    display: flex;
    align-items: center;
}

.time-bar {
    height: 0.3rem;
    background-color: #ffffff;
    border-radius: 0.15rem;
    display: inline-block;
    vertical-align: middle;
    margin-right: 0.5rem;
}
</style>
