<template>
    <div class="app-content">
        <div class="sidebar-parent">
            <SidebarComponent :sortMode="sortMode" @sort_mode_changed="sortModeChanged" />
        </div>
        <div class="apps-parent">
            <AppsComponent :sortMode="sortMode" />
        </div>
    </div>
</template>

<script>
import { ref } from 'vue';
import SidebarComponent from './components/SidebarComponent.vue';
import AppsComponent from './components/AppsComponent.vue';

export default {
    components: {
        SidebarComponent,
        AppsComponent,
    },
    setup() {
        const sortMode = ref("millis_in_foreground");

        const sortModeChanged = (newSortMode) => {
            sortMode.value = newSortMode;
        };

        return {
            sortMode,
            sortModeChanged,
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
    background-color: #757575;

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
}

.apps-parent {
    display: flex;
    justify-content: center;
    width: 100%;
    max-height: 98vh;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: #ffffff rgba(0, 0, 0, 0.1);
}
</style>
