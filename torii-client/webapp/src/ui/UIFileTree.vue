<template>
    <UITree
        ref="uiTreeEl"
        class="ui-file-tree"
        v-model:selected-keys="selectedKeys"
        :key="JSON.stringify(files)"
        :data="files"
        :actions="[{ label: 'Remove', key: 'remove', action: removeRecordUI }]"
        @find-node-by-key="findNodeByKey"
        @node-click="(e) => setCurrentFile(e)"
        @node-expand="(e) => loadNodes(e.value.relative_path)"
        @node-move="moveRecord"
        @node-rename="renameRecordUI"
        generic="Record"
    />
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { Record } from '../types';
import UITree, { TreeNode } from './UITree.vue';
import { listRecordComponents, listRecords, removeRecord, renameRecord, saveRecordComponent } from '@/services/recordsService.ts';

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
const filter = ref<string | undefined>(undefined);

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
 * @param directory The directory to load the nodes from.
 * @param recursive Whether to load the nodes recursively. This will return a flattened
 * list of records. The tree structure will not be preserved. Defaults to `false`.
 */
async function loadNodes(
    directory: string,
    recursive: boolean = false
): Promise<TreeNode<Record>[]> {
    /**
     * @brief Converts a record object into a treenode object.
     * @param record The record to convert.
     * @returns A `TreeNode` object representing the record.
     */
    async function mapRecord(record: Record): Promise<TreeNode<Record>> {
        const components = await listRecordComponents(record);

        console.debug(`Components for \`${record.name}\`:`, components);
        const isFolder = components.some((c) => c.name === 'folder');

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
        const files = await listRecords(props.workspace, directory, recursive);
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

    // Create folder component for parent record if it doesn't exist, then move
    // the record into new path.
    await saveRecordComponent(parent, 'folder', '', 'text/markdown');
    const newRecord = await renameRecord(record, `${parent.relative_path}/${record.name}`);

    console.log(`Moved record \`${record.name}\` to \`${parent.name}\`:`, newRecord);

    await refresh();
}

async function removeRecordUI(node: TreeNode<Record>) {
    const record = node.value;
    await removeRecord(record);
    await refresh();
}

async function renameRecordUI(node: TreeNode<Record>, newLabel: string) {
    const record = node.value;
    if (!record) return;

    // Wether the current record is "active", meaning we currently view
    // it in the editor.
    const isActive = selectedKeys.value.includes(record.path);

    const newRelativePathSlice = record.relative_path.split('/');
    /* const _oldName = */ newRelativePathSlice.pop();
    newRelativePathSlice.push(newLabel);
    const newRelativePath = newRelativePathSlice.join('/');

    // Move the record to the new parent path.
    const newRecord = await renameRecord(record, newRelativePath);

    // If the current record is active, update the "selected key" to the new
    // record's path.
    if (isActive) {
        const selectedIndex = selectedKeys.value.indexOf(record.path);
        if (selectedIndex !== -1) {
            selectedKeys.value[selectedIndex] = newRecord.path;
            emit('update:current-file', newRecord);
        }
    }

    console.log(`Renamed record \`${record.name}\` to \`${newRecord.name}\``);
    await refresh();
}

/**
 * Refresh the file tree after moving the record.
 */
async function refresh() {
    const newNodes = await loadNodes('');

    /**
     * Recurse over all nodes, updating "data" while preserving
     * "UI state" (expanded/collapsed, selected, etc.).
     */
    async function refreshRecursive(
        newNodes: TreeNode<Record>[],
        oldNodes: TreeNode<Record>[]
    ): Promise<TreeNode<Record>[]> {
        const nodes: TreeNode<Record>[] = [];

        for (const node of newNodes) {
            const oldNode = oldNodes.find((n) => n.key === node.key);
            if (!oldNode) {
                nodes.push(node);
                continue;
            }

            // Synchronize the "isOpened" state from the old node to the new node and
            // reload the children.
            if (oldNode.isOpened && oldNode.children) {
                const newNodes = await loadNodes(node.value.relative_path);
                const oldNodes = oldNode.children ?? [];
                node.children = await refreshRecursive(newNodes, oldNodes);
                node.isOpened = true;
            }

            nodes.push(node);
        }

        return nodes;
    }

    if (!filter.value) {
        files.value = await refreshRecursive(newNodes, files.value);
    } else {
        const allRecords = await loadNodes(props.workspace, true);
        files.value = allRecords.filter((node) =>
            node.value.name.toLowerCase().includes(filter.value!.toLowerCase())
        );
    }
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
 * Sets the filter for the file tree. If a filter is set, only nodes that match
 * the filter will be displayed.
 * @param newFilter A string to filter the file tree by. If undefined, the filter is cleared.
 */
function setFilter(newFilter: string | undefined) {
    filter.value = newFilter;
    refresh();
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

    // Try to find "Readme.md" in the root directory and select it if found.
    const readmeNode = files.value.find(
        (node) => node.value.name.toLocaleUpperCase() === 'README'
    );
    if (readmeNode) setCurrentFile(readmeNode);
});

defineExpose({
    currentRecord,
    getFiles,
    refresh,
    selectKeys,
    setFilter,
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
