<template>
    <div class="view-project">
        <div class="view-project-side">
            <FileTree
                ref="fileTree"
                :root="projectPath"
                @update:current-file="setCurrentFile"
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
                :key="currentFile.directory + '/' + currentFile.name"
                :directory="markdownDirectory"
                :name="markdownName"
                component="image"
                @refresh="loadComponents()"
                v-if="currentFile"
            />
            <MarkdownEditor
                :directory="markdownDirectory"
                :name="markdownName"
                :autocomplete-suggestion="(v) => autocompleteMarkdown(v)"
                :placeholder="!recordComponents.includes('article')"
                @open-file="openFile"
                v-if="currentFile"
            />
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import FileTree, { Record } from '../../components/file/FileTree.vue';

// Components
import MarkdownEditor from '../../components/article/MarkdownEditor.vue';
import ImageEditor from '../../components/image/ImageEditor.vue';

const route = useRoute();
const router = useRouter();
const projectPath = route.query.project as string;
const currentFile = ref<{ directory: string; name: string } | null>(null);
const markdownDirectory = ref<string | null>(null);
const markdownName = ref<string | null>(null);
const recordComponents = ref<string[]>([]);
const fileTree = ref<InstanceType<typeof FileTree> | null>(null);
const records = ref<Record[]>([]);

onMounted(async () => {
    const files = await fileTree.value?.loadFiles();
    const readme = files?.find((r) => r.name === 'README');
    records.value = files || [];

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

        if (fileTree.value) {
            fileTree.value.setCurrentFile(newFile);
        }
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

    console.log('Components listed:', recordComponents.value);
}

async function autocompleteMarkdown(name: string): Promise<any> {
    return records.value
        .filter((record) => record.name.startsWith(name))
        .map((record) => ({
            label: record.name,
            value: record.name,
        }));
}

async function openFile(name: string) {
    const record = records.value.find((r) => r.name === name);
    if (!record) return;

    currentFile.value = record;
}

function setCurrentFile(file: Record | null) {
    if (!file) return;
    currentFile.value = file;
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
