<template>
    <div class="sidebar">
        <select v-model="selectedSortMode" @change="handleSortModeChange">
            <option value="millis_in_foreground">millis_in_foreground</option>
            <option value="millis_in_background">millis_in_background</option>
            <option value="times_opened">times_opened</option>
            <option value="times_focused">times_focused</option>
        </select>
    </div>
</template>

<script>
import { ref } from 'vue';

export default {
    props: {
        sortMode: String,
    },
    setup(props, context) {
        const selectedSortMode = ref(props.sortMode);

        const handleSortModeChange = (event) => {
            context.emit("sort_mode_changed", event.target.value);
        };

        return {
            selectedSortMode,
            handleSortModeChange,
        };
    },
};
</script>

<style scoped>
.sidebar {
    display: flex;
    justify-content: center;
    margin-top: 20px;
}

select {
    padding: 10px;
    font-size: 16px;
    border-radius: 5px;
    border: 1px solid #ccc;
    outline: none;
}
</style>
