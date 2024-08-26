<template>
    <div class="apps">
        <div class="app" v-for="app in apps" :key="app.display_name" @click="openAppDetails(app.name)">
            <div class="app-name">{{ app.display_name }}</div>
            <div class="app-time">
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
import { onBeforeUnmount, onMounted, ref, watch, watchEffect } from 'vue';
import { calculateDisplayValue } from '../utils/timeUtils.js';

export default {
    props: {
        sortMode: String,
        date: String,
    },
    setup(props, context) {
        const updateIntervalMillis = 3000;
        const apps = ref([]);
        let intervalId = null;

        const sortMode = ref(props.sortMode);
        const date = ref(props.date);

        const getApps = async () => {
            let newApps = [];

            const sortedApps = await invoke("get_screen_time_apps_sorted", { date: date.value, sortMode: sortMode.value, reversed: true });

            for (let app of sortedApps) {
                switch (props.sortMode) {
                    case "millis_in_foreground":
                        newApps.push({
                            name: app.name,
                            display_name: app.display_name,
                            millis_in_foreground: app.millis_in_foreground[date.value],
                            display_value: calculateDisplayValue(app.millis_in_foreground[date.value]),
                        });
                        break;
                    case "millis_in_background":
                        newApps.push({
                            name: app.name,
                            display_name: app.display_name,
                            millis_in_background: app.millis_in_background[date.value],
                            display_value: calculateDisplayValue(app.millis_in_background[date.value]),
                        });
                        break;
                    case "times_opened":
                        newApps.push({
                            name: app.name,
                            display_name: app.display_name,
                            times_opened: app.times_opened[date.value],
                            display_value: app.times_opened[date.value],
                        });
                        break;
                    case "times_focused":
                        newApps.push({
                            name: app.name,
                            display_name: app.display_name,
                            times_focused: app.times_focused[date.value],
                            display_value: app.times_focused[date.value],
                        });
                        break;
                }
            }

            apps.value = newApps;
        };

        const calculatePercentage = (value) => {
            const maxValue = Math.max(...apps.value.map(app => app[props.sortMode]));
            const percentage = (value / maxValue) * 100;
            return percentage;
        };

        const openAppDetails = (appName) => {
            context.emit("open_app_details", appName);
        };

        const startUpdateLoop = async () => {
            intervalId = setInterval(async () => {
                await getApps();
            }, updateIntervalMillis);
        };

        watch(props, async () => {
            sortMode.value = props.sortMode;
            date.value = props.date;
            await getApps();
        });

        onMounted(async () => {
            await getApps();
            startUpdateLoop();
        });

        onBeforeUnmount(() => {
            clearInterval(intervalId);
            intervalId = null;
        });

        return { apps, sortMode, date, getApps, calculatePercentage, openAppDetails };
    }
};
</script>

<style>
.apps {
    margin: 1rem;
    height: fit-content;
    display: grid;
    grid-template-columns: repeat(1, 1fr);
    gap: 1rem;
    width: 75%;
}

.app {
    padding: 1rem 2rem;
    background: #4c5566;
    border: 2px solid #687691;
    border-radius: 0.5rem;
    cursor: pointer;
    user-select: none;
    height: fit-content;
}

.app:hover {
    background: #59657e;
}

.app div {
    white-space: nowrap;
}

.app-name {
    font-size: 1.4rem;
}

.app-time {
    display: flex;
    align-items: center;
    color: #dbdbdb;
}

.time-bar {
    height: 0.3rem;
    background-color: #dbdbdb;
    border-radius: 0.15rem;
    display: inline-block;
    vertical-align: middle;
    margin-right: 0.5rem;
}
</style>
