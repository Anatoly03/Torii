<template>
    <div class="view-recent-projects">
        <span class="no-projects" v-if="recentProjects.length == 0">
            {{ $t('app.project.noRecent') }}
        </span>
        <div v-else class="project-list">
            <span
                :tabindex="index"
                @keydown.space.enter ="openProject(project.path)"
                class="project-entry"
                v-for="(project, index) in recentProjects"
                :key="project.path"
            >
                <span
                    @click="openProject(project.path)"
                    class="project-title"
                    >{{ project.name }}</span
                >
                <n-dropdown
                    trigger="hover"
                    placement="right"
                    :options="getProjectOptions(project)"
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
        <button @click="openProjectDialog">
            {{ $t('app.project.open') }}
        </button>
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
import { useRouter } from 'vue-router';

const router = useRouter();
const recentProjects = listRecentProjects();

function getProjectOptions(project: any): DropDownOption[] {
    if (project.is_system) {
        return [{ label: 'Remove', key: 'remove', disabled: true }];
    }

    return [{ label: 'Remove', key: 'remove' }];
}

/**
 * Opens a dialog to open a project folder from file system.
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

/**
 * Opens the project at the given path.
 */
function openProject(projectPath: string) {
    router.push({ name: 'project', query: { project: projectPath } });
}

/**
 * @param project Project Metadata
 * @param option Project Option
 */
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
            align-items: center;
            height: 2em;
            cursor: pointer;

            &:hover {
                background-color: #eee;
            }

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
