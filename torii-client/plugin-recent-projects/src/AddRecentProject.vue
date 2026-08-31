<template>
    <button class="add-recent-project" @click="openProjectDialog">
        {{ $t('app.project.open') }}
    </button>
</template>

<script lang="ts" setup>
import { open } from '@tauri-apps/plugin-dialog';
import { addRecentProject } from './service';

/**
 * @brief Opens a dialog to open a project folder from file system.
 */
async function openProjectDialog() {
    const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select a project folder',
    });

    if (!selected || typeof selected !== 'string') return;

    const name = selected.split('/').pop() || 'Unknown Project';
    addRecentProject({ path: selected, name });
}
</script>

<style lang="scss" scoped>
.add-recent-project {
    padding: 8px 12px;
    border: none;
    background-color: #f0f0f0;
    border-radius: 4px;
    text-align: left;
    cursor: pointer;

    &:hover {
        background-color: #e0e0e0;
    }

    &:disabled {
        background-color: #f0f0f0;
        color: #999;
        cursor: not-allowed;
    }
}
</style>
