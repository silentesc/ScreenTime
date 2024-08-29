<script>
import { onMounted, ref } from 'vue';
import ModalComponent from './ModalComponent.vue';
import { invoke } from '@tauri-apps/api';
export default {
    components: {
        ModalComponent,
    },
    setup(props, context) {
        const apps = ref([]);
        const appsSuccess = ref([]);
        const appsError = ref([]);

        const closeModal = () => {
            context.emit('close_settings');
        };

        const changeDisplayName = async (app) => {
            const success = await invoke("change_display_name", { appPath: app.path, newDisplayName: app.display_name });
            if (success) {
                appsSuccess.value.push(app.path);
                setTimeout(() => {
                    appsSuccess.value = appsSuccess.value.filter((path) => path !== app.path);
                }, 3000);
            }
            else {
                console.error('Failed to change display name for app:', app.name);
                appsError.value.push(app.path);
                setTimeout(() => {
                    appsError.value = appsError.value.filter((path) => path !== app.path);
                }, 3000);
            }
        };

        const changeHidden = async (app) => {
            const success = await invoke("change_hidden", { appPath: app.path, hidden: app.hidden });
            if (!success) {
                console.error('Failed to change hidden status for app:', app.name);
                app.hidden = !app.hidden;
            }
        };

        onMounted(async () => {
            apps.value = await invoke('get_screen_time_apps');
        });

        return {
            apps,
            appsSuccess,
            appsError,
            closeModal,
            changeDisplayName,
            changeHidden,
        };
    },
};
</script>

<template>
    <ModalComponent :modalWidth="'80%'" @close_modal="closeModal">
        <h1>Settings</h1>
        <div class="settings" v-if="apps">
            <div class="settings-item" v-for="app in apps" :key="app">
                <input class="checkbox-hidden" type="checkbox" :id="app.path" v-model="app.hidden" @change="changeHidden(app)">
                <label class="label-hidden" :for="app.path">Hide</label>
                <input
                    :class="{ 'display-name': true, 'app-success': appsSuccess.includes(app.path), 'app-error': appsError.includes(app.path) }"
                    type="text" v-model="app.display_name" spellcheck="false" @keypress.enter="changeDisplayName(app)">
            </div>
        </div>
    </ModalComponent>
</template>

<style scoped>
.settings {
    display: grid;
    grid-template-columns: repeat(1, 1fr);
    gap: 0.7rem;
}

.settings-item {
    display: flex;
    align-items: center;
}

.checkbox-hidden {
    width: 1.5rem;
    height: 1.5rem;
    cursor: pointer;
}

.label-hidden {
    margin-right: 1.5rem;
    font-size: 1.2rem;
    cursor: pointer;
}

.display-name {
    width: 40%;
    font-size: 1.2rem;
    background-color: #31363F;
    color: #fff;
    border: 2px solid #687691;
    border-radius: 0.25rem;
    outline: none;
    padding: 0.5rem;
}

.display-name:focus {
    background-color: #2a2e33;
}

.app-success {
    border: 2px solid #75ff75;
}

.app-error {
    border: 2px solid #ff6d6d;
}
</style>
