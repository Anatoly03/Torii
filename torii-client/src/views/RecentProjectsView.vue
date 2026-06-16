<template>
    <div class="view-recent-projects">
        <span class="no-projects" v-if="recentProjects.length == 0">
            {{ $t('placeholder.noRecentProjects') }}
        </span>
        <div v-else class="project-list">
            <button v-for="project in recentProjects" :key="project.path" disabled>
                {{ project.name }}
            </button>
        </div>
        <button @click="openProjectDialog">Open Project</button>
    </div>
</template>

<script setup lang="ts">
import { listRecentProjects, addRecentProject } from '../composables/recentProjects';
import { open } from '@tauri-apps/plugin-dialog';

const recentProjects = listRecentProjects();
console.log(recentProjects);

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
.view-recent-projects {
    display: flex;
    flex-direction: column;
    gap: 16px;
    margin-bottom: 16px;
    font-size: 14px;
    color: #666;
    flex: 1;

    .project-list {
        display: flex;
        flex-direction: column;
        gap: 8px;

        button {
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
    }
}
</style>
