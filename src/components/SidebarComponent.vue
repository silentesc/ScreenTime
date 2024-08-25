<template>
    <div class="sidebar">
        <label>Select Date</label>
        <div class="date-div">
            <button class="arrow-left" @click="addDaysToDate(-1)">{{ "<" }}</button>
            <span class="date" @click="resetDate">{{ selectedDate }}</span>
            <button class="arrow-right" @click="addDaysToDate(1)">{{ ">" }}</button>
        </div>

        <br>

        <label>Sorting Mode</label>
        <select v-model="selectedSortMode" @change="handleSortModeChange">
            <option value="millis_in_foreground">Focus Time</option>
            <option value="millis_in_background">Background Time</option>
            <option value="times_opened">Times Opened</option>
            <option value="times_focused">Times Focused</option>
        </select>

        <span class="settings-emoji" @click="handleSettingsClick">⚙️</span>
    </div>
</template>

<script>
import { ref } from 'vue';
import { getCurrentDate, getDateWithOffset, isDateStrValid } from '../utils/dateUtils.js';

export default {
    props: {
        sortMode: String,
        date: String,
    },
    setup(props, context) {
        const selectedSortMode = ref(props.sortMode);
        const selectedDate = ref(props.date);

        const handleSortModeChange = (event) => {
            context.emit("sort_mode_changed", event.target.value);
        };

        const handleSettingsClick = () => {
            context.emit("settings_clicked");
        };

        const addDaysToDate = (days) => {
            const newDate = getDateWithOffset(selectedDate.value, days);
            if (!isDateStrValid(newDate)) {
                return;
            }
            selectedDate.value = newDate;
            context.emit("date_changed", selectedDate.value);
        };

        const resetDate = () => {
            const newDate = getCurrentDate();
            if (selectedDate.value === newDate) {
                return;
            }
            selectedDate.value = newDate;
            context.emit("date_changed", newDate);
        };

        return {
            selectedSortMode,
            selectedDate,
            handleSortModeChange,
            handleSettingsClick,
            addDaysToDate,
            resetDate,
        };
    },
};
</script>

<style scoped>
.sidebar {
    display: flex;
    flex-direction: column;
    padding: 1rem;
    justify-content: center;
    margin-top: 20px;
}

select {
    padding: 0.5rem;
    font-size: 1rem;
    border-radius: 0.25rem;
    border: 1px solid #687691;
    background-color: #31363F;
    color: #fff;
    cursor: pointer;
    outline: none;
}

.date-div {
    display: flex;
}

.date {
    font-size: 1.4rem;
    padding: 0.5rem;
    background-color: transparent;
    border-top: 1px solid #687691;
    border-bottom: 1px solid #687691;
    display: inline-block;
    cursor: pointer;
}

button {
    background-color: #31363F;
    border: 1px solid #687691;
    border-radius: 0.25rem;
    color: #fff;
    cursor: pointer;
    user-select: none;
}

.arrow-left {
    font-size: 1.4rem;
    border-radius: 0.25rem 0rem 0rem 0.25rem;
}

.arrow-right {
    font-size: 1.4rem;
    border-radius: 0rem 0.25rem 0.25rem 0rem;
}

.settings-emoji {
    font-size: 2.5rem;
    position: absolute;
    bottom: 1rem;
    left: 0.5rem;
    cursor: pointer;
    user-select: none;
}
</style>
