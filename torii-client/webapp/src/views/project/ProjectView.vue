<template>
    <div class="view-project">
        <div class="view-project-sidebar">
            <div class="view-project-quick-actions">
                <button @click="createNewFile()">
                    <Icon>
                        <CreateOutline />
                    </Icon>
                </button>
                <input
                    type="text"
                    placeholder="Search..."
                    v-model="searchQuery"
                />
                <button @click="fileTree?.refresh()">
                    <Icon>
                        <RefreshOutline />
                    </Icon>
                </button>
            </div>
            <UIFileTree
                class="view-project-sidebar-file-tree"
                :workspace="projectPath"
                ref="fileTree"
                @update:current-file="setCurrentFile"
            />
            <!-- <FileTree
                ref="fileTree"
                :root="projectPath"
                @update:current-file="setCurrentFile"
            /> -->
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
                    :record="currentFile"
                    component="banner"
                    :placeholder-text="$t('app.project.bannerPlaceholder')"
                    placeholder-anchor="left"
                    class="view-project-banner"
                    :view-mode="settings.viewMode"
                    @refresh="loadComponents()"
                    v-if="currentFile"
                />
                <ImageEditor
                    :key="currentFile.directory + '/' + currentFile.name"
                    :record="currentFile"
                    component="image"
                    placeholder-anchor="center"
                    class="view-project-image"
                    :view-mode="settings.viewMode"
                    @refresh="loadComponents()"
                    v-if="currentFile"
                />
                <div
                    class="view-project-content-body"
                    :style="{
                        paddingTop: needsExtraPadding ? '176px' : '0',
                    }"
                >
                    <MarkdownEditor
                        ref="markdownEditor"
                        v-model:word-count="wordCount"
                        :record="currentFile"
                        :autocomplete-start="autocompleteStart"
                        :autocomplete-suggestion="
                            (v) => autocompleteMarkdown(v)
                        "
                        :placeholder="!recordComponents.includes('article')"
                        :view-mode="settings.viewMode"
                        @open-file="openFile"
                        v-if="currentFile"
                    />
                </div>
            </div>
            <div class="view-project-footer">
                <div class="view-project-footer-left">
                    <span
                        class="view-record-word-count"
                        v-if="settings.enableWordCount"
                    >
                        {{ $t('app.project.wordCount', { count: wordCount }) }}
                    </span>
                </div>
                <div class="view-project-footer-right">
                    <span class="view-mode">
                        <UISelect
                            :options="[
                                { label: 'Edit', value: 'edit' },
                                { label: 'Preview', value: 'preview' },
                            ]"
                            v-model="settings.viewMode"
                        />
                    </span>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { Icon } from '@vicons/utils';
import {
    SettingsOutline,
    CreateOutline,
    RefreshOutline,
} from '@vicons/ionicons5';
import { openSettingsWindow } from '../../composables/settings-window.ts';
import { useSettingsStore } from '@/stores/settings';

// Components
import MarkdownEditor from '../../components/article/MarkdownEditor.vue';
import ImageEditor from '../../components/image/ImageEditor.vue';
import { Record } from 'types';
import UIFileTree from '@/ui/UIFileTree.vue';
import UISelect from '@/ui/UISelect.vue';
import { TreeNode } from 'ui/UITree.vue';
import { usePlugins } from '@/services/plugins.ts';

const route = useRoute();
const router = useRouter();
const settings = useSettingsStore();
const searchQuery = ref<string>('');
const projectPath = route.query.project as string;
const currentFile = ref<Record | null>(null);
const markdownDirectory = ref<string | null>(null);
const markdownName = ref<string | null>(null);
const recordComponents = ref<string[]>([]);
const fileTree = ref<InstanceType<typeof UIFileTree> | null>(null);
const markdownEditor = ref<InstanceType<typeof MarkdownEditor> | null>(null);
const records = ref<Record[]>([]);
const autocompleteCache = ref<Record[]>([]);
const wordCount = ref<number | undefined>(undefined);

const needsExtraPadding = computed(() => {
    return (
        recordComponents.value.includes('banner') &&
        !recordComponents.value.includes('image')
    );
});

onMounted(async () => {
    const nodes = await fileTree.value?.getFiles();
    const files: Record[] = nodes.map((k) => k.value) ?? [];
    const readme = files?.find((r) => r.name === 'README');
    records.value = files || [];

    if (readme) {
        currentFile.value = readme;
    }

    await loadComponents();

    usePlugins().emit('workspace-open', projectPath);
});

onUnmounted(() => {
    usePlugins().emit('workspace-close', projectPath);
});

watch(currentFile, (newFile) => {
    if (newFile) {
        console.log(newFile);
        markdownDirectory.value = newFile.workspace.path;
        markdownName.value = newFile.name;
        loadComponents();

        fileTree.value?.selectKeys([newFile.path]);
    } else {
        markdownDirectory.value = null;
        markdownName.value = null;
    }
});

watch(searchQuery, (query) => {
    if (!fileTree.value) return;
    fileTree.value.setFilter(query.length ? query : undefined);
});

async function loadComponents() {
    if (!currentFile.value) return;
    console.log('Loading components for:', currentFile.value);

    const recordComponentsResult = await invoke<any[]>(
        'list_record_components',
        {
            record: currentFile.value,
        }
    );
    recordComponents.value = recordComponentsResult.map((c) => c.name);

    console.log('Components listed:', recordComponents.value);
}

async function createNewFile() {
    if (!fileTree.value) return;

    const files: TreeNode<Record>[] = await fileTree.value.getFiles();

    // If there are no files, we create "README" as the first file.
    if (files.length === 0) {
        const projectName = projectPath.split('/').pop() ?? 'My Project';

        await invoke<string>('save_record_component', {
            record: {
                workspace: projectPath,
                relative_path: 'README',
            },
            component: 'article',
            content: `# ${projectName}\n\nWelcome to your new project! This is the README file, which you can edit into the first record.`,
            contentType: 'text/markdown',
        });

        fileTree.value.refresh();
        return;
    }

    // If we're not in a fresh start, we create a new file with a unique name.
    let newFileName = 'New File';
    let counter = 1;

    while (files.some((file) => file.value.name === newFileName)) {
        newFileName = `New File ${counter}`;
        counter++;
    }

    await invoke<string>('save_record_component', {
        record: {
            workspace: projectPath,
            relative_path: newFileName,
        },
        component: 'article',
        content: `# ${newFileName}\n\n`,
        contentType: 'text/markdown',
    });

    fileTree.value.refresh();
}

function listRecords(attrs: {
    directory: string;
    recursive?: boolean;
    filter?: string;
}) {
    return invoke<Record[]>('list_records', {
        workspace: projectPath,
        directory: attrs.directory,
        recursive: attrs.recursive ?? false,
        filter: attrs.filter,
    });
}

/**
 * On autocomplete popup creation, cache all records.
 */
async function autocompleteStart() {
    const suggestions = await listRecords({
        directory: '',
        recursive: true,
    });
    autocompleteCache.value = suggestions;
}

/**
 * Filter caches records by name and return them as autocomplete suggestions.
 * @param name The name to filter records by.
 */
async function autocompleteMarkdown(filter: string): Promise<any> {
    if (!currentFile.value) return [];

    return autocompleteCache.value
        .filter((record) => {
            return record.name.toLowerCase()?.startsWith(filter.toLowerCase());
        })
        .map((record) => {
            // In order to generate a "proper" relative link for value, we nee to first find
            // which file we're in and navigate to the workspace root, relative to the current
            // file.
            const workspaceRelative =
                currentFile.value?.relative_path
                    .split('/')
                    .map((_, index) => (index == 0 ? '.' : '..'))
                    .join('/') ?? '.';
            // Then we append the record's relative path to the workspace root.
            const relativePath = `${workspaceRelative}/${record.relative_path}`;

            return {
                label: record.name,
                value: relativePath,
            };
        });
}

async function openFile(record: Record) {
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

        .view-project-quick-actions {
            display: flex;
            flex-direction: row;

            // width: 100%;
            box-sizing: border-box;
            // align-items: center;
            gap: 8px;
            min-height: 2em;

            input {
                flex: 1;
                min-width: 0;
            }
        }

        .view-project-sidebar-file-tree {
            flex: 1;
            overflow-y: auto;
        }

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

            &.disabled {
                pointer-events: none;
                border: none;

                &.no-content {
                    display: none;
                }
            }
        }

        .view-project-image {
            position: relative;
            min-width: 200px;
            min-height: 200px;
            max-width: 200px;
            max-height: 200px;
            z-index: 10;
            border: 2px dashed #ccc;
            border-radius: 8px;
            margin: 16px;
            background-color: #fafafaaa;

            &.disabled {
                pointer-events: none;
                border: none;

                &.no-content {
                    display: none;
                }
            }
        }
    }

    .view-project-content-body {
        width: 100%;
    }

    .view-project-footer {
        display: flex;
        flex-direction: row;
        // align-items: center;
        justify-content: space-between;
        padding: 8px;
        border-top: 1px solid #ccc;

        .view-project-footer-left {
            align-self: flex-start;
        }

        .view-project-footer-right {
            align-self: flex-end;
        }
    }
}
</style>
