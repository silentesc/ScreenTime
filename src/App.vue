<template>
    <div class="app-content" v-if="isAppOpen">
        <div class="sidebar-parent">
            <SidebarComponent :sortMode="sortMode" :date="date" @sort_mode_changed="sortModeChanged"
                @date_changed="date_changed" />
        </div>
        <div class="apps-parent">
            <AppsComponent :sortMode="sortMode" :date="date" @open_app_details="openAppDetails" />
        </div>
    </div>
    <div v-if="openedAppDetail">
        <AppDetailComponent :appName="openedAppDetail" :date="date" @close_app_details="closeAppDetails" />
    </div>
</template>

<script>
import { onMounted, ref } from 'vue';
import SidebarComponent from './components/SidebarComponent.vue';
import AppsComponent from './components/AppsComponent.vue';
import AppDetailComponent from './components/AppDetailComponent.vue';
import { getCurrentDate } from './utils/dateUtils.js';
import { appWindow } from '@tauri-apps/api/window';

export default {
    components: {
        SidebarComponent,
        AppsComponent,
        AppDetailComponent,
    },
    setup() {
        const sortMode = ref("millis_in_foreground");
        const date = ref(getCurrentDate());
        const openedAppDetail = ref(null);

        const isAppOpen = ref(false);

        const listenForAppClose = async () => {
            await appWindow.onResized(async () => {
                isAppOpen.value = await appWindow.isVisible();
                if (!isAppOpen.value) {
                    closeAppDetails();
                }
            });
        };
        listenForAppClose();

        const sortModeChanged = (newSortMode) => {
            sortMode.value = newSortMode;
        };

        const date_changed = (newDate) => {
            date.value = newDate;
        };

        const openAppDetails = (appName) => {
            openedAppDetail.value = appName;
        };

        const closeAppDetails = () => {
            openedAppDetail.value = null;
        };

        onMounted(() => {
            isAppOpen.value = true;
        });

        return {
            sortMode,
            date,
            openedAppDetail,
            isAppOpen,
            sortModeChanged,
            date_changed,
            openAppDetails,
            closeAppDetails,
        };
    },
};
</script>

<style>
:root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #fff;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
}

.app-content {
    display: flex;
}

.sidebar-parent {
    width: 250px;
    height: 100vh;
    background-color: #222831;
}

.apps-parent {
    display: flex;
    justify-content: center;
    width: 100%;
    max-height: 100vh;
    background-color: #31363F;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: #ffffff rgba(0, 0, 0, 0.1);
}
</style>
