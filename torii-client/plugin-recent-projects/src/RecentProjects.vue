<template>
    <div class="view-recent-projects">
        <span class="no-projects" v-if="recentProjects.length == 0">
            {{ $t('app.project.noRecent') }}
        </span>
        <div v-else class="project-list">
            <RecentProjectItem
                :tabindex="1"
                v-for="(project, index) in recentProjects"
                :key="project.path"
                :project="project"
                @open="openProject(project.path)"
            />
        </div>
        <AddRecentProject />
    </div>
</template>

<script setup lang="ts">
import { listRecentProjects } from './service.ts';
import { useRouter } from 'vue-router';
import AddRecentProject from './AddRecentProject.vue';
import RecentProjectItem from './RecentProjectItem.vue';

const router = useRouter();
const recentProjects = listRecentProjects();

/**
 * @brief Opens the project at the given path.
 */
function openProject(projectPath: string) {
    router.push({ name: 'project', query: { project: projectPath } });
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
    }
}
</style>
