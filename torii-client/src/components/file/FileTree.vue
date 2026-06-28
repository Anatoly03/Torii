<!-- Naive UI Tree: https://www.naiveui.com/en-US/os-theme/components/tree -->

<template>
    <div class="view-file-tree">
        <n-tree
            :align="'left'"
            :value="currentFile"
            :data="treeData"
            :render-label="renderLabel"
            :render-prefix="(_) => null"
            :render-suffix="renderFileOptions"
            :render-switcher-icon="(_) => null"
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
    EllipsisVertical,
} from '@vicons/ionicons5';
import { NIcon, NTree, NInput, NButton, NSpace, NDropdown } from 'naive-ui';
import { Component, h, onMounted, ref, nextTick, watch, VNodeChild } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import FileTreeCreateFile from './FileTreeCreateFile.vue';
import { TreeRenderProps } from 'naive-ui/es/tree/src/interface';

interface Record {
    directory: string;
    name: string;
}

const props = defineProps<{
    root: string;
}>();

const emit = defineEmits<{
    (e: 'file-selected', path?: { directory: string; name: string }): void;
    (e: 'file-created', path: { directory: string; name: string }): void;
    (
        e: 'update:current-file',
        path: { directory: string; name: string } | null
    ): void;
}>();

const treeData = ref<(TreeOption & { record: Record })[]>([]);
const currentFile = ref<{ directory: string; name: string } | null>();
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
    return h(
        'span',
        {
            tabindex: 1,
            onKeydown: (e: KeyboardEvent) => {
                if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    onSelectFile([option.key as string]);
                }
            },
        },
        option.label
    );
}

function renderFileOptions(tree_props: TreeRenderProps): VNodeChild {
    if (tree_props.option.key === NEW_FILE_KEY) {
        return null; // No options for the new file input
    }

    return h(
        NDropdown,
        {
            trigger: 'hover',
            placement: 'right',
            options: [
                {
                    label: 'Delete',
                    key: 'delete',
                },
            ],
            showArrow: true,
            onSelect: async (_, option) => {
                if (option.key === 'delete') {
                    let record = tree_props.option.record as any;
                    await invoke('remove_record_component', {
                        directory: record.directory,
                        name: record.name,
                        component: 'article',
                    });
                    await refreshFiles();
                }
            },
        },
        {
            default: () =>
                h(
                    NIcon,
                    { style: { color: '#ff4d4f' } },
                    {
                        default: () => h(EllipsisVertical),
                    }
                ),
        }
    );
}

function onSelectFile(keys: string[]) {
    const fileKeys = keys.filter((key) => key !== NEW_FILE_KEY);

    if (!fileKeys.length) {
        currentFile.value = null;
        emit('file-selected', undefined);
        return;
    }

    const key = fileKeys[0]!;
    const data = treeData.value.find((node) => node.key === key)!;

    currentFile.value = data.record;
    emit('update:current-file', currentFile.value);
    emit('file-selected', currentFile.value);
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
            record: null!,
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

        await invoke<string>('save_record_component', {
            directory: props.root,
            name,
            component: 'article',
            content: `# ${name}\n\n- Source: ${path}\n- Parent: ${props.root}\n- Name: ${name}\n\n`,
        });

        // Refresh the file list
        await refreshFiles();

        // Emit event
        emit('file-created', {
            directory: props.root,
            name,
        });

        // Select the new file
        emit('file-selected', {
            directory: props.root,
            name,
        });

        // Reset state
        cancelNewFile();
    } catch (error) {
        console.error('Failed to create file:', error);
        // You might want to show an error notification here
    } finally {
        isLoading.value = false;
    }
}

async function refreshFiles(): Record[] {
    if (!props.root) return;

    try {
        // Lists all records.
        const files = await invoke<Record[]>('list_records', {
            directory: props.root,
        });

        const newData = files.map((file) => {
            return {
                record: file,
                key: file.directory + '/' + file.name,
                label: file.name,
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
                    record: null!,
                },
            ];
        } else {
            treeData.value = newData;
        }

        return files;
    } catch (error) {
        console.error('Failed to list files:', error);
    }

    return [];
}

/**
 * Exposed method to load files, can be called from parent component
 * to refresh the file tree and list all records.
 */
function loadFiles() {
    return refreshFiles();
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
