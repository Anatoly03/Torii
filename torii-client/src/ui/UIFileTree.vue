<template>
    <UITree
        ref="uiTreeEl"
        class="ui-file-tree"
        v-model:selected-keys="selectedKeys"
        :key="JSON.stringify(files)"
        :data="files"
        :actions="[{ label: 'Remove', key: 'remove', action: removeRecord }]"
        @find-node-by-key="findNodeByKey"
        @node-click="(e) => setCurrentFile(e)"
        @node-expand="(e) => loadNodes(e.value.relative_path)"
        @node-move="moveRecord"
        generic="Record"
    />
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { Record } from '../types.d.ts';
import UITree, { TreeNode } from './UITree.vue';
import { invoke } from '@tauri-apps/api/core';

/**
 *
 */
const props = defineProps<{
    workspace: string;
}>();

/**
 *
 */
const emit = defineEmits<{
    (e: 'update:current-file', value: Record | null): void;
}>();

/**
 * The reference to the UITree component instance.
 */
const uiTreeEl = ref<InstanceType<typeof UITree> | null>(null);

/**
 * @brief Model for the selected keys in the tree. Propagated from the UITree
 * component to the parent and backwards.
 */
const selectedKeys = defineModel<string[]>('selectedKeys', {
    default: () => [] as string[],
});

/**
 * Compute the current record.
 */
const currentRecord = computed(() => {
    const selectedKey = selectedKeys.value[0];
    if (!selectedKey) return null;
    return findNodeByKey(files.value, selectedKey)?.value ?? null;
});

/**
 * @brief The loaded file hierarchy, represented as an tree of `TreeNode` objects.
 */
const files = ref<TreeNode<Record>[]>([]);

/**
 * @brief Loads the file nodes from the backend.
 */
async function loadNodes(directory: string): Promise<TreeNode<Record>[]> {
    /**
     * @brief Converts a record object into a treenode object.
     * @param record The record to convert.
     * @returns A `TreeNode` object representing the record.
     */
    async function mapRecord(record: Record): Promise<TreeNode<Record>> {
        const components = await invoke<string[]>('list_record_components', {
            record,
        });

        console.debug(`Components for \`${record.name}\`:`, components);
        const isFolder = components.some((c) => c === 'folder');

        return {
            value: record,
            key: record.path,
            label: record.name,
            isLeaf: !isFolder,
        };
    }

    /**
     * @brief Sorts the tree nodes by their record label in ascending order.
     */
    function sortNodes(records: TreeNode<Record>[]): TreeNode<Record>[] {
        return records.sort((a, b) => a.label.localeCompare(b.label));
    }

    try {
        const files = await invoke<Record[]>('list_records', { 
            workspace: props.workspace,
            directory,
            recursive: false,
        });
        const retrieved = await Promise.all(files.map(mapRecord));
        return sortNodes(retrieved);
    } catch (e) {
        console.error('Error loading nodes:', e);
    }
    return [];
}

function setCurrentFile(node: TreeNode<Record>) {
    const record = node.value;
    if (!record) return;
    emit('update:current-file', record);
}

async function moveRecord(
    parentNode: TreeNode<Record>,
    node: TreeNode<Record>
) {
    const parent = parentNode.value;
    const record = node.value;
    if (!parent || !record) return;

    // Create folder component for parent record if it doesn't exist.
    await invoke('save_record_component', {
        record: parent,
        component: 'folder',
        content: '',
        contentType: 'text/markdown',
    });

    console.log(record);

    // Move the record to the new parent path.
    const newRecord = await invoke<Record>('rename_record', {
        record,
        newName: `${parent.relative_path}/${record.name}`,
    });

    console.log(
        `Moved record \`${record.name}\` to new parent \`${parent.name}\`:`,
        newRecord
    );

    await refresh();
}

async function removeRecord(node: TreeNode<Record>) {
    const record = node.value;
    await invoke('remove_record', { record });
    await refresh();
}

/**
 * Refresh the file tree after moving the record.
 */
async function refresh() {
    files.value = await loadNodes('');
}

/**
 * @brief Recursively finds a node by its key in the tree.
 * @param key The key of the node to find.
 * @param nodes The nodes to search in. Defaults to the root nodes.
 * @returns The found node or null if not found.
 */
function findNodeByKey(
    key: string,
    nodes = files.value
): TreeNode<Record> | null {
    // console.log('Searching for node with key:', key);

    for (const node of nodes) {
        if (node.key === key) return node;
        if (node.children) {
            const found = findNodeByKey(key, node.children);
            if (found) return found;
        }
    }
    return null;
}

/**
 * Wrapper, exposed to the parent, for setting the current file by its record.
 * @param record The record to set as the current file.
 */
function getFiles() {
    // TODO reload the file tree from the backend before returning
    return files.value;
}

/**
 * @brief Wrapper, exposed to the parent, for selecting keys in the tree.
 * @param keys The keys to select.
 */
function selectKeys(keys: string[]) {
    selectedKeys.value = keys;
}

onMounted(async () => {
    await refresh();
});

defineExpose({
    currentRecord,
    getFiles,
    refresh,
    selectKeys,
});
</script>

<style lang="scss" scoped>
.ui-file-tree {
    box-sizing: border-box;
    // overflow-x: hidden;
    overflow-y: auto;
    padding-right: 8px;
}
</style>
