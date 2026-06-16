<template>
    <div class="view-recent-projects">
        <span class="no-projects" v-if="recentProjects.length == 0">
            {{ $t('placeholder.noRecentProjects') }}
        </span>
        <div v-else class="project-list">
            <span
                class="project-entry"
                v-for="project in recentProjects"
                :key="project.path"
            >
                <div class="project-title">{{ project.name }}</div>
                <n-dropdown
                    trigger="click"
                    placement="right"
                    :options="projectOptions"
                    :show-arrow="true"
                    @select="
                        (_, option) => handleProjectOptions(project, option)
                    "
                >
                    <n-icon class="project-remove">
                        <ellipsis-vertical />
                    </n-icon>
                </n-dropdown>
            </span>
        </div>
        <button @click="openProjectDialog">Open Project</button>
    </div>
</template>

<script setup lang="ts">
import {
    listRecentProjects,
    addRecentProject,
    removeRecentProject,
} from '../composables/recentProjects';
import { open } from '@tauri-apps/plugin-dialog';
import { NIcon, NDropdown, DropdownOption } from 'naive-ui';
import { EllipsisVertical } from '@vicons/ionicons5';

const recentProjects = listRecentProjects();
const projectOptions: DropdownOption[] = [{ label: 'Remove', key: 'remove' }];

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

function handleProjectOptions(
    project: { path: string; name: string },
    option: DropdownOption
) {
    console.log('Selected option', option, 'for project', project);
    switch (option.key) {
        case 'remove':
            removeRecentProject(project.path);
            break;
    }
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

        .project-entry {
            display: flex;
            flex-direction: row;

            .project-title {
                flex: 1;
            }

            .project-remove {
                cursor: pointer;
                color: #999;

                &:hover {
                    color: #666;
                }
            }
        }

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
