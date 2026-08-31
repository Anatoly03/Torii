<template>
    <div
        class="button-update"
        :class="{ active: isActive, disabled: isFinished }"
        @click="nextAction()"
    >
        <span class="content"> {{ message }} </span>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { check, Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

const message = ref('Checking for Update...');
const isActive = ref(true);
const isFinished = ref(false);
const nextAction = ref(() => {});

async function checkForUpdate() {
    try {
        const update = await check();
        if (!update) {
            message.value = 'Up to Date';
            isFinished.value = true;
            return;
        }

        console.debug(`Update to Version ${update.version} available:`, update);
        message.value = 'Install ' + update.version;
        isActive.value = false;
        nextAction.value = installUpdate(update);
    } catch (e) {
        console.warn(e);
        message.value = 'Updater not Available';
        isFinished.value = true;
    }
}

function installUpdate(update: Update) {
    return async () => {
        isActive.value = true;

        let downloaded = 0;
        let contentLength: number | undefined = 0;

        await update.download((event) => {
            switch (event.event) {
                case 'Started':
                    contentLength = event.data.contentLength;
                    message.value = `Downloading (0/${contentLength})...`;
                    break;
                case 'Progress':
                    downloaded += event.data.chunkLength;
                    message.value = `Downloading (${downloaded}/${contentLength})...`;
                    break;
                // case 'Finished':
                //     break;
            }
        });

        message.value = 'Installing...';
        await update.install();

        message.value = 'Relaunch';
        nextAction.value = () => relaunch();
        isActive.value = false;
    };
}

onMounted(() => checkForUpdate());
</script>

<style lang="scss" scoped>
.button-update {
    display: flex;
    flex-direction: column;
    justify-content: center;
    width: 180px;
    height: 28px;
    border: 1px solid #ccc;
    border-radius: 4px;
    background-color: #eee;
    vertical-align: middle;

    &:not(.active) {
        cursor: pointer;
    }

    &.active {
        cursor: wait;
    }

    &:hover,
    &.active {
        background-color: #ddd;
    }

    &.disabled {
        cursor: not-allowed;
        background-color: #ccc;
    }
}
</style>
