<template>
    <div class="modal" @mousedown.self="closeModal">
        <div class="modal-content" v-if="app">
            <div class="app-name">
                {{ app.display_name }}
            </div>
            <hr>
            <div class="grid">
                <div class="grid-item">
                    Path:
                </div>
                <div class="grid-item">
                    <a @click="openPath">{{ app.path }}</a>
                </div>
                <div class="grid-item">
                    Focus Time:
                </div>
                <div class="grid-item">
                    {{ app.millis_in_foreground }}
                </div>
                <div class="grid-item">
                    Background Time:
                </div>
                <div class="grid-item">
                    {{ app.millis_in_background }}
                </div>
                <div class="grid-item">
                    Times Opened:
                </div>
                <div class="grid-item">
                    {{ app.times_opened }}
                </div>
                <div class="grid-item">
                    Times Focused:
                </div>
                <div class="grid-item">
                    {{ app.times_focused }}
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api';
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { calculateDisplayValue } from '../utils/timeUtils.js';

export default {
    props: {
        appName: String,
        date: String,
    },
    setup(props, context) {
        const updateIntervalMillis = 1000;
        let intervalId = null;
        const app = ref(null);

        const closeModal = () => {
            context.emit("close_app_details");
        };

        const getApp = async () => {
            app.value = await invoke("get_screen_time_app_by_name", { appName: props.appName, ignoreCase: false });
            app.value.path = app.value.path;
            app.value.millis_in_foreground = calculateDisplayValue(app.value.millis_in_foreground[props.date] || 0);
            app.value.millis_in_background = calculateDisplayValue(app.value.millis_in_background[props.date] || 0);
            app.value.times_opened = app.value.times_opened[props.date] || 0;
            app.value.times_focused = app.value.times_focused[props.date] || 0;
        };

        const startUpdateLoop = async () => {
            intervalId = setInterval(async () => {
                await getApp();
            }, updateIntervalMillis);
        };

        const openPath = () => {
            invoke("open_path", { path: app.value.path });
        };

        onMounted(async () => {
            await getApp();
            startUpdateLoop();
        });

        onBeforeUnmount(() => {
            clearInterval(intervalId);
            intervalId = null;
        });

        return {
            app,
            closeModal,
            getApp,
            openPath,
        };
    },
}
</script>

<style scoped>
.modal {
    display: block;
    position: fixed;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
}

.modal-content {
    background-color: #4c5566;
    margin: 15% auto;
    padding: 20px;
    border-radius: 5px;
    width: 60%;
    font-size: 1.2rem;
}

.app-name {
    font-size: 2rem;
    margin: 1rem;
}

.grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-gap: 1rem;
    margin: 1rem 0;
}

.grid-item {
    width: 100%;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.grid-item:hover {
    white-space: normal;
    word-break: break-all;
}

a {
    cursor: pointer;
}
a:hover {
    text-decoration: underline;
}
</style>
