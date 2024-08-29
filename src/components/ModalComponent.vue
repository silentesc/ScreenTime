<script>
export default {
    props: {
        modalWidth: {
            type: String,
            default: "60%",
        },
    },
    setup(props, context) {
        const closeModal = () => {
            context.emit("close_modal");
        };

        return {
            closeModal,
        };
    },
}
</script>

<template>
    <div class="modal" @mousedown.self="closeModal">
        <div class="modal-content" :style="{ width: modalWidth }">
            <slot></slot>
        </div>
    </div>
</template>

<style scoped>
.modal {
    display: block;
    position: fixed;
    align-content: center;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    max-height: 100vh;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: #ffffff rgba(0, 0, 0, 0.1);
}

.modal-content {
    background-color: #4c5566;
    margin: auto auto;
    padding: 20px;
    border-radius: 5px;
    font-size: 1.2rem;
    margin-top: 1rem;
    margin-bottom: 1rem;
}
</style>
