<template>
    <div class="view-project">
        <div class="view-project-sidebar">
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
                <button
                    class="view-home-settings-button"
                    @click="openSettingsWindow()"
                >
                    <Icon><SettingsOutline /></Icon>
                </button>
            </div>
        </div>
        <div class="view-project-layout">
            <div class="view-project-content" @scroll="onScrollProjectContent">
                <ImageEditor
                    :key="currentFile.directory + '/' + currentFile.name"
                    :directory="markdownDirectory"
                    :name="markdownName"
                    component="banner"
                    :placeholder-text="$t('app.project.bannerPlaceholder')"
                    placeholder-anchor="left"
                    class="view-project-banner"
                    @refresh="loadComponents()"
                    v-if="currentFile"
                />
                <ImageEditor
                    :key="currentFile.directory + '/' + currentFile.name"
                    :directory="markdownDirectory"
                    :name="markdownName"
                    component="image"
                    placeholder-anchor="center"
                    class="view-project-image"
                    @refresh="loadComponents()"
                    v-if="currentFile"
                />
                <MarkdownEditor
                    ref="markdownEditor"
                    v-model:word-count="wordCount"
                    :directory="markdownDirectory"
                    :name="markdownName"
                    :autocomplete-suggestion="(v) => autocompleteMarkdown(v)"
                    :placeholder="!recordComponents.includes('article')"
                    @open-file="openFile"
                    v-if="currentFile"
                />
            </div>
            <div class="view-project-footer">
                <span class="view-record-word-count">
                    {{ $t('app.project.wordCount', { count: wordCount }) }}
                </span>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import FileTree, { Record } from '../../components/file/FileTree.vue';
import { Icon } from '@vicons/utils';
import { SettingsOutline } from '@vicons/ionicons5';
import { openSettingsWindow } from '../../composables/settings-window.ts';

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
const markdownEditor = ref<InstanceType<typeof MarkdownEditor> | null>(null);
const records = ref<Record[]>([]);
const wordCount = ref<number | undefined>(undefined);

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

function onScrollProjectContent(event: Event) {
    markdownEditor.value?.onScrollProjectContent(event);
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

    .view-project-sidebar {
        display: flex;
        flex-direction: column;
        width: 200px;
        padding: 16px;
        gap: 8px;
        border-right: 1px solid #ccc;

        .view-project-quick-settings {
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 8px;
            min-height: 2em;

            .view-project-return-to-menu {
                flex: 1;
            }

            .view-home-settings-button {
                flex: 0;
            }

            button {
                flex: 1;
            }
        }
    }

    .view-project-layout {
        display: flex;
        flex-direction: column;
        flex: 1;
    }

    // .view-project-content-placeholder {
    //     display: flex;
    //     flex-direction: column;
    //     align-items: center;
    //     justify-content: center;
    //     flex: 1;
    // }

    .view-project-content {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-start;
        flex: 1;
        text-align: left;
        overflow-y: auto;

        .view-project-banner {
            position: absolute;
            width: 100%;
            height: 176px;
            border-bottom: 2px dashed #ccc;
            overflow: hidden;
        }

        .view-project-image {
            position: relative;
            min-width: 200px;
            min-height: 200px;
            z-index: 10;
            border: 2px dashed #ccc;
            border-radius: 8px;
            margin: 16px;
            background-color: #fafafaaa;
        }
    }

    .view-project-footer {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: start;
        padding: 8px;
        border-top: 1px solid #ccc;
    }
}
</style>
