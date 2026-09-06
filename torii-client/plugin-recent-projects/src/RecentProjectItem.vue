<template>
    <div class="recent-project-item" @click="openProject()" @keydown.space.enter="openProject()">
        <span class="project-title">{{ project.name }}</span>
        <n-dropdown
            trigger="hover"
            placement="right"
            :options="projectOptions"
            :show-arrow="true"
            @select="(_, option) => handleProjectOption(option)"
        >
            <n-icon class="project-remove">
                <ellipsis-vertical />
            </n-icon>
        </n-dropdown>
    </div>
</template>

<script setup lang="ts">
import { RecentProject, removeRecentProject } from './service';
import { NIcon, NDropdown, DropdownOption } from 'naive-ui';
import { EllipsisVertical } from '@vicons/ionicons5';
import { computed } from 'vue';

const props = defineProps<{
    project: RecentProject;
}>();
const emit = defineEmits<{
    open: [RecentProject];
}>();

/**
 * @brief The project options, which appear in the dropdown. To specify behaviour
 * based on the dropdown action, refer to {@link handleProjectOption}.
 */
const projectOptions = computed<DropdownOption[]>(() => {
    if (props.project.is_system) {
        return [{ label: 'Remove', key: 'remove', disabled: true }];
    }

    return [{ label: 'Remove', key: 'remove' }];
});

/**
 * @param project Project Metadata
 * @param option Project Option
 */
function handleProjectOption(option: DropdownOption) {
    switch (option.key) {
        case 'remove':
            removeRecentProject(props.project.path);
            break;
        default:
            console.error('Not Implemented: Received drop down action:', option.key, option);
    }
}

/**
 * @brief Opens the project for the current recent project item.
 */
function openProject() {
    emit('open', props.project);
}
</script>

<style lang="scss" scoped>
.recent-project-item {
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
</style>
