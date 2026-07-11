<template>
    <div class="ui-tree">
        <ul v-if="data && data.length" class="ui-tree-list">
            <li v-for="node of data" :key="node.key">
                <n-spin
                    v-if="node.isLoadingChildren"
                    :size="14"
                    class="ui-tree-anchor-spinner"
                    :show="true"
                />
                <NIcon
                    v-else
                    class="ui-tree-list-anchor"
                    @click="onToggleAnchor(node)"
                >
                    <ChevronDown v-if="!node.isLeaf && node.opened" />
                    <ChevronForward v-else-if="!node.isLeaf" />
                </NIcon>
                <span class="ui-tree-list-content">
                    <UIDropRegion
                        class="ui-tree-list-item"
                        draggable
                        :drag-data="[['application/x-record-path', node.key]]"
                        @drop-application-record-path="onMoveNode(node, $event)"
                    >
                        <span
                            class="ui-tree-list-label"
                            :class="{
                                'is-selected': selectedKeys.includes(node.key),
                            }"
                            @click="selectNode(node)"
                        >
                            <span class="ui-tree-list-label-text">
                                {{ node.label }}
                            </span>
                            <NDropdown
                                class="ui-tree-list-actions"
                                v-if="props.actions"
                                trigger="hover"
                                placement="right"
                                show-arrow
                                @select="props.actions.find((a) => a.key === $event)?.action(node)"
                                :options="props.actions"
                            >
                                <NIcon style="color: #ff4d4f">
                                    <EllipsisVertical />
                                </NIcon>
                            </NDropdown>
                        </span>
                    </UIDropRegion>
                    <UITree
                        v-if="
                            node.opened && node.children && node.children.length
                        "
                        :data="node.children"
                        :actions="props.actions"
                        v-model:selected-keys="selectedKeys"
                        @find-node-by-key="
                            (s) => props.onFindNodeByKey?.(s) ?? null
                        "
                        @node-click="emit('node-click', $event)"
                        @node-expand="(node) => props.onNodeExpand(node)"
                        @node-move="(s, t) => emit('node-move', s, t)"
                    />
                </span>
            </li>
        </ul>
    </div>
</template>

<script setup lang="ts" generic="NodeValue">
import { Ref, ref } from 'vue';
import {
    ChevronDown,
    ChevronForward,
    EllipsisVertical,
} from '@vicons/ionicons5';
import { NDropdown, NIcon, NSpin } from 'naive-ui';
import UIDropRegion from './UIDropRegion.vue';

/**
 * @brief A generic tree node type that can be used to represent hierarchical
 * data in a tree structure.
 * @details Required attributes are `key` (which is unique in the whole tree),
 * `label` (which is the text displayed in the tree), and `value` which is used
 * in callbacks.
 *
 * Optionally, `isLeaf` can be set to true to indicate that the node is a leaf
 * node and cannot contain children. `opened` is set by the tree to indicate
 * collapsed nodes. `children` is an array of child nodes, which can be empty
 * or undefined for leaf nodes.
 */
export type TreeNode<K> = {
    key: string;
    label: string;
    value: K;
    isLeaf?: boolean;
    isLoadingChildren?: boolean;
    opened?: boolean;
    children?: TreeNode<K>[];
};

/**
 *
 */
export type TreeDropdownOption<K> = {
    label: string;
    key: string;
    action: (node: TreeNode<K>) => void;
};

/**
 * @brief Properties of the UITree component.
 * @details The `data` property is an array of `TreeNode` objects that represent
 * the data displayed in the tree root.
 */
const props = defineProps<{
    data: TreeNode<NodeValue>[];

    /**
     * @brief Actions (suffix) for the tree nodes. If provided, these actions will
     * be displayed in a dropdown menu when the user right-clicks on a node.
     * @param node
     */
    actions?: TreeDropdownOption<NodeValue>[];

    /**
     * // TODO document
     */
    onNodeExpand: (node: TreeNode<K>) => Promise<TreeNode<K>[]>;

    /**
     * @brief Callback function provided by the root node to find a node by its key.
     * @details The reason this method is implemented by the caller is because drag
     * and drop to the
     * @param key The key of the record.
     */
    onFindNodeByKey?: (key: string) => TreeNode<NodeValue> | null;
}>();

/**
 * @brief Emits events from the UITree component.
 */
const emit = defineEmits<{
    (e: 'node-click', node: TreeNode<NodeValue>): void;
    (
        e: 'node-move',
        target: TreeNode<NodeValue>,
        source: TreeNode<NodeValue>
    ): void;
    // (e: 'node-expand', node: TreeNode<NodeValue>): Promise<TreeNode<NodeValue>[]>;
}>();

/**
 * @brief Model for the selected keys in the tree.
 */
const selectedKeys = defineModel<string[]>('selectedKeys', {
    default: () => [] as string[],
});

/**
 * @brief The data displayed in the tree, taken from the `data` property and
 * managed by the component.
 */
const data = ref<TreeNode<NodeValue>[]>(props.data);

/**
 * Is triggered when a node is clicked, updates the selected keys and emits a
 * `node-click` event with the selected keys.
 * @param node The node that was clicked.
 * @todo Allow shift+click for multiple selections.
 */
function selectNode(node: TreeNode<NodeValue>) {
    selectedKeys.value = [node.key];
    // emit('node-click', selectedKeys.value);
    // console.log('Node clicked:', node);
    emit('node-click', node);
}

/**
 * Toggles the anchor state and loads child nodes if necessary.
 */
async function onToggleAnchor(node: TreeNode<NodeValue>) {
    const state = (node.opened = !node.opened);

    if (state && !node.isLeaf && !node.children && props.onNodeExpand) {
        node.isLoadingChildren = true;
        node.children = await props.onNodeExpand(node);
        node.isLoadingChildren = false;
        console.debug(
            `Loaded children for node \`${node.label}\`:`,
            node.children
        );
    }
}

/**
 * // TODO DOCUMENT
 * @param node
 * @param recordPath
 */
function onMoveNode(target: TreeNode<NodeValue>, sourceRecordPath: string) {
    if (!props.onFindNodeByKey) return;

    const source = props.onFindNodeByKey?.(sourceRecordPath) ?? null;
    if (!source) return; // if the path is externally dragged, ignore

    emit('node-move', target, source);
}

defineOptions({ name: 'UITree' });
</script>

<style lang="scss" scoped>
.ui-tree {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
        sans-serif;
    font-size: 14px;
    color: #333;
    width: 100%;
    box-sizing: border-box;
    // user-select: none; // prevent accidental text selection

    ul.ui-tree-list {
        width: 100%;
        display: flex;
        box-sizing: border-box;
        flex-direction: column;
        list-style-type: none;
        padding: 0;
        margin: 0;
        gap: 4px;

        li {
            display: flex;
            flex-direction: row;
            gap: 4px;
            max-width: 100%; // full width
            max-width: 100%; // prevent overflow
            box-sizing: border-box;
            cursor: pointer;

            .ui-tree-anchor-spinner {
                font-size: 0.2em;
                flex-shrink: 0;
            }

            .ui-tree-list-anchor {
                aspect-ratio: 1;
                flex-shrink: 0;
                font-size: 1em;
                margin-top: 5px;
                color: #888;
            }

            .ui-tree-list-content {
                flex: 1;
                min-width: 0;
                width: 100%;
                display: flex;
                flex-direction: column;
                justify-content: flex-start;
                text-align: left;
                width: 100%;
            }

            .ui-tree-list-label {
                display: flex;
                width: 100%;
                box-sizing: border-box;
                padding: 4px;
                border-radius: 4px;
                align-items: center;
                white-space: nowrap; // keep label on one line
                overflow: hidden; // hide overflow
                text-overflow: ellipsis; // show … when truncated
                user-select: none; // prevent accidental text selection

                &.is-selected {
                    background-color: #e0e0e0;

                    &:hover {
                        background-color: #d0d0d0;
                    }
                }

                &:hover {
                    background-color: #f0f0f0;
                }

                .ui-tree-list-label-text {
                    flex: 1;
                    min-width: 0;
                    overflow: hidden; // hide overflow
                    text-overflow: ellipsis; // show … when truncated
                }

                .ui-tree-list-actions {
                    display: flex;
                    flex-direction: row;
                }
            }

            :v-deep(.ui-tree) {
                width: 100%;
            }
        }
    }
}
</style>
