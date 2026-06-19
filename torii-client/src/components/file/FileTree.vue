<!-- Naive UI Tree: https://www.naiveui.com/en-US/os-theme/components/tree -->

<template>
    <div class="view-file-tree">
        <n-tree
            :align="'left'"
            :value="currentFile"
            :data="treeData"
            :render-label="renderLabel"
            :render-prefix="_ => null"
            :render-switcher-icon="_ => null"
            block-line
            block-node
            @update:selected-keys="onSelectFile"
        >
            <template #empty> Empty workspace. </template>
        </n-tree>
        <button @click="startNewFile" class="new-file-btn">
            <NIcon><AddOutline /></NIcon>
            New File
        </button>
    </div>
</template>

<script setup lang="ts">
import type { TreeOption } from 'naive-ui';
import {
    FileTrayFullOutline,
    AddOutline,
    CloseOutline,
} from '@vicons/ionicons5';
import { NIcon, NTree, NInput, NButton, NSpace } from 'naive-ui';
import { Component, h, onMounted, ref, nextTick, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import FileTreeCreateFile from './FileTreeCreateFile.vue';

const props = defineProps<{
    root: string;
}>();

const emit = defineEmits<{
    (e: 'file-selected', path: string): void;
    (e: 'file-created', path: string): void;
    (e: 'update:current-file', path: string | null): void;
}>();

const treeData = ref<TreeOption[]>([]);
const currentFile = ref<string | null>();
const isLoading = ref(false);
const isCreatingNew = ref(false);

watch(currentFile, (newFile) => {
    emit('update:current-file', newFile ?? null);
});

// Special key for the "new file" input node
const NEW_FILE_KEY = '__new_file__';

function createIcon(icon: Component) {
    return () =>
        h(NIcon, null, {
            default: () => h(icon),
        });
}

function renderLabel({ option }: { option: TreeOption }) {
    // Special rendering for the "new file" input row
    if (option.key === NEW_FILE_KEY) {
        return h(
            'div',
            {
                style: {
                    display: 'flex',
                    alignItems: 'center',
                    gap: '8px',
                    width: '100%',
                },
            },
            h(FileTreeCreateFile, {
                onCreate: createFile,
                onCancel: cancelNewFile,
            })
        );
    }

    // Regular file label
    return option.label;
}

function onSelectFile(keys: string[]) {
    if (keys.length > 0) {
        const key = keys[0];
        if (key !== NEW_FILE_KEY) {
            currentFile.value = key;
            emit('file-selected', key);
        }
    }
}

function startNewFile() {
    if (isCreatingNew.value) return; // Prevent multiple inputs
    isCreatingNew.value = true;

    // Add the placeholder node to the tree
    treeData.value = [
        ...treeData.value,
        {
            key: NEW_FILE_KEY,
            label: '', // Will be rendered by renderLabel
            isLeaf: true,
            prefix: createIcon(FileTrayFullOutline),
        },
    ];

    // Focus the input after render
    nextTick(() => {
        const input = document.querySelector('.n-input input');
        if (input) {
            (input as HTMLInputElement).focus();
        }
    });
}

function cancelNewFile() {
    isCreatingNew.value = false;
    // Remove the placeholder node
    treeData.value = treeData.value.filter((node) => node.key !== NEW_FILE_KEY);
}

async function createFile(name: string) {
    if (!name) return;

    isLoading.value = true;
    try {
        const path = props.root.endsWith('/')
            ? props.root + name + '.md'
            : props.root + '/' + name + '.md';
        const filePath = await invoke<string>('save_markdown_file', {
            path,
            content: `# ${name}\n\nSource: \`${path}\`\n\n`,
        });

        // Refresh the file list
        await refreshFiles();

        // Emit event
        emit('file-created', filePath);

        // Select the new file
        emit('file-selected', filePath);

        // Reset state
        cancelNewFile();
    } catch (error) {
        console.error('Failed to create file:', error);
        // You might want to show an error notification here
    } finally {
        isLoading.value = false;
    }
}

async function refreshFiles() {
    if (!props.root) return;

    try {
        const files = await invoke<any[]>('list_markdown_files', {
            directory: props.root,
        });

        const newData = files.map((file) => {
            const name = file.path.split('/').pop() || file.path;
            const name_no_ext = name.split('.').slice(0, -1).join('.');

            return {
                key: file.path,
                label: name_no_ext,
                isLeaf: true,
                prefix: createIcon(FileTrayFullOutline),
            };
        });

        // Preserve the "new file" input if it's active
        if (isCreatingNew.value) {
            treeData.value = [
                ...newData,
                {
                    key: NEW_FILE_KEY,
                    label: '',
                    isLeaf: true,
                    prefix: createIcon(FileTrayFullOutline),
                },
            ];
        } else {
            treeData.value = newData;
        }
    } catch (error) {
        console.error('Failed to list files:', error);
    }
}

function loadFiles() {
    refreshFiles();
}

// Expose refresh method
defineExpose({
    loadFiles,
    refreshFiles,
    currentFile,
});

onMounted(() => {
    loadFiles();
});
</script>

<style lang="scss" scoped>
.view-file-tree {
    display: flex;
    width: 100%;
    flex-direction: column;
    gap: 16px;
    margin-bottom: 16px;
    font-size: 14px;
    color: #666;
    flex: 1;
}

.new-file-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    border: 1px dashed #ccc;
    border-radius: 4px;
    background: transparent;
    color: #666;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
        border-color: #42b983;
        color: #42b983;
        background: #f0faf5;
    }
}

:deep(.n-tree) {
    .n-tree-node {
        &.n-tree-node--selected {
            background-color: #e8f5e9;
        }
    }
}

:deep(.n-input) {
    flex: 1;
}
</style>
