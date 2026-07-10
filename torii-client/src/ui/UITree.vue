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
                    <UIDropRegion>
                        <span
                            class="ui-tree-list-label"
                            :class="{
                                'is-selected': selectedKeys.includes(node.key),
                            }"
                            @click="selectNode(node)"
                        >
                            {{ node.label }}
                        </span>
                    </UIDropRegion>
                    <UITree
                        v-if="
                            node.opened && node.children && node.children.length
                        "
                        :data="node.children"
                        v-model:selected-keys="selectedKeys"
                        @node-click="emit('node-click', $event)"
                    />
                </span>
            </li>
        </ul>
    </div>
</template>

<script setup lang="ts" generic="NodeValue">
import { Ref, ref } from 'vue';
import { ChevronDown, ChevronForward } from '@vicons/ionicons5';
import { NIcon, NSpin } from 'naive-ui';
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
 * @brief Properties of the UITree component.
 * @details The `data` property is an array of `TreeNode` objects that represent
 * the data displayed in the tree root.
 */
const props = defineProps<{
    data: TreeNode<NodeValue>[];
    onNodeExpand: (node: TreeNode<K>) => Promise<TreeNode<K>[]>;
}>();

/**
 * @brief Emits events from the UITree component.
 */
const emit = defineEmits<{
    (e: 'node-click', node: TreeNode<NodeValue>): void;
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

    if (state && !node.isLeaf && !node.children) {
        node.isLoadingChildren = true;
        node.children = await props.onNodeExpand(node);
        node.isLoadingChildren = false;
        console.debug(`Loaded children for node \`${node.label}\`:`, node.children);
    }

    console.log(node);
}

defineOptions({ name: 'UITree' });
</script>

<style lang="scss" scoped>
.ui-tree {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
        sans-serif;
    font-size: 14px;
    color: #333;
    // user-select: none; // prevent accidental text selection

    ul.ui-tree-list {
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
            cursor: pointer;

            .ui-tree-anchor-spinner {
                font-size: 0.2em;
            }

            .ui-tree-list-anchor {
                aspect-ratio: 1;
                font-size: 1em;
                margin-top: 5px;
                color: #888;
            }

            .ui-tree-list-content {
                flex: 1;
                flex-direction: column;
                justify-content: flex-start;
                text-align: left;
                width: 100%;
            }

            .ui-tree-list-label {
                display: flex;
                width: 100%;
                padding: 4px;
                border-radius: 4px;
                align-items: center;

                &.is-selected {
                    background-color: #e0e0e0;

                    &:hover {
                        background-color: #d0d0d0;
                    }
                }

                &:hover {
                    background-color: #f0f0f0;
                }
            }
        }
    }
}
</style>
