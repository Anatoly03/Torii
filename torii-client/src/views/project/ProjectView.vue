<template>
    <div class="view-project">
        <div class="view-project-side">
            <span class="view-project-file-tree"
                >Project View File Navigation</span
            >
            <div class="view-project-quick-settings">
                <button
                    class="view-project-return-to-menu"
                    @click="$router.push('/')"
                >
                    Close Project
                </button>
            </div>
        </div>
        <div class="view-project-content">Project View: {{ projectPath }}</div>
    </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';

const route = useRoute();
const projectPath = route.params.projectPath as string;
</script>

<style lang="scss" scoped>
.view-project {
    display: flex;
    flex-direction: row;
    height: 100%;

    .view-project-side {
        display: flex;
        flex-direction: column;
        width: 200px;
        padding: 16px;
        gap: 8px;
        border-right: 1px solid #ccc;

        .view-project-file-tree {
            display: flex;
            flex-direction: column;
            gap: 16px;
            margin-bottom: 16px;
            font-size: 14px;
            color: #666;
            flex: 1;
        }

        .view-project-quick-settings {
            display: flex;
            flex-direction: row;
            gap: 8px;
            min-height: 2em;

            button {
                flex: 1;
            }
        }
    }

    .view-project-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        flex: 1;
    }
}
</style>
