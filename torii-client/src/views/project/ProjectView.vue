<template>
    <div class="view-project">
        <div class="view-project-side">
            <FileTree
                ref="fileTree"
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
        <!-- <div class="view-project-content-placeholder" v-if="!currentFile">
            Project View: {{ projectPath }}<br />
        </div> -->
        <div class="view-project-content">
            <ImageEditor
                :directory="markdownDirectory"
                :name="markdownName"
                @refresh="loadComponents()"
                v-if="currentFile && recordComponents.includes('image')"
            />
            <MarkdownEditor
                :directory="markdownDirectory"
                :name="markdownName"
                v-if="currentFile && recordComponents.includes('article')"
            />
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import FileTree from '../../components/file/FileTree.vue';
import MarkdownEditor from './MarkdownEditor.vue';
import ImageEditor from './ImageEditor.vue';

const route = useRoute();
const router = useRouter();
const projectPath = route.params.projectPath as string;
const currentFile = ref<{ directory: string; name: string } | null>(null);
const markdownDirectory = ref<string | null>(null);
const markdownName = ref<string | null>(null);
const recordComponents = ref<string[]>([]);
const fileTree = ref<InstanceType<typeof FileTree> | null>(null);

onMounted(async () => {
    const files = await fileTree.value?.loadFiles();
    const readme = files?.find((r) => r.name === 'README');

    if (readme) {
        currentFile.value = readme;
    }

    loadComponents();
});

watch(currentFile, (newFile) => {
    if (newFile) {
        markdownDirectory.value = newFile.directory;
        markdownName.value = newFile.name;
        loadComponents();
    } else {
        markdownDirectory.value = null;
        markdownName.value = null;
    }
});

async function loadComponents() {
    if (!currentFile.value) return;

    const { directory, name } = currentFile.value;
    recordComponents.value = await invoke('list_record_components', {
        directory,
        name,
    });
}

if (!projectPath) {
    // If no project path is provided, redirect to the home page
    router.push('/');
}
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
        align-items: center;
        justify-content: flex-start;
        flex: 1;
        text-align: left;
        overflow-y: auto;
    }
}
</style>
