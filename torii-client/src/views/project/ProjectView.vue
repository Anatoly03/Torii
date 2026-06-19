<template>
    <div class="view-project">
        <div class="view-project-side">
            <FileTree
                :root="projectPath"
                @update:current-file="currentFile = $event"
            />
            <div class="view-project-quick-settings">
                <button
                    class="view-project-return-to-menu"
                    @click="$router.push('/')"
                >
                    {{ $t('app.project.close') }}
                </button>
            </div>
        </div>
        <div class="view-project-content-placeholder" v-if="!currentFile">
            Project View: {{ projectPath }}<br />
        </div>
        <div class="view-project-content" v-else>
            <pre>{{ textBody }}</pre>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import FileTree from '../../components/file/FileTree.vue';
import { invoke } from '@tauri-apps/api/core';

const route = useRoute();
const router = useRouter();
const projectPath = route.params.projectPath as string;
const currentFile = ref<string | null>(null);
const textBody = ref<string>('');

if (!projectPath) {
    // If no project path is provided, redirect to the home page
    router.push('/');
}

watch(currentFile, async (path) => {
    if (!path) return;

    // Load the file content when a new file is selected
    let content = await invoke<string>('get_markdown_file', { path });
    textBody.value = content;
});
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

    .view-project-content-placeholder {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        flex: 1;
    }

    .view-project-content {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        justify-content: flex-start;
        flex: 1;
        margin: 16px;
        text-align: left;
    }
}
</style>
