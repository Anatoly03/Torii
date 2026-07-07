<template>
    <div class="ui-file-tree">
        <ul v-if="data && data.length" class="ui-file-tree-list">
            <li v-for="node of data" :key="node.key">
                <NIcon
                    class="ui-file-tree-list-anchor"
                    @click="node.opened = !node.opened"
                >
                    <ChevronDown v-if="!node.isLeaf && node.opened" />
                    <ChevronForward v-else-if="!node.isLeaf" />
                </NIcon>
                <span class="ui-file-tree-list-content">
                    <UIDropRegion>
                        <span
                            class="ui-file-tree-list-label"
                            :class="{
                                'is-selected': selectedKeys.includes(node.key),
                            }"
                            @click="selectNode(node)"
                        >
                            {{ node.label }}
                        </span>
                    </UIDropRegion>
                    <UIFileTree
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
import { NIcon } from 'naive-ui';
import UIDropRegion from './UIDropRegion.vue';

type TreeNode = {
    key: string;
    label: string;
    value: NodeValue;
    isLeaf?: boolean;
    opened?: boolean;
    children?: TreeNode[];
};

const props = defineProps<{
    data?: TreeNode[];
}>();

const emit = defineEmits<{
    (e: 'node-click', value: string[]): void;
}>();

const selectedKeys = defineModel<string[]>('selectedKeys', {
    default: () => [] as string[],
});

const data = ref(
    props.data ?? [
        { key: 'node1', label: 'Node 1', opened: false, isLeaf: true },
        { key: 'node2', label: 'Node 2', opened: false, isLeaf: true },
        {
            key: 'node3',
            label: 'Node 3',
            opened: false,
            children: [
                {
                    key: 'node3-1',
                    label: 'Node 3.1',
                    opened: false,
                    isLeaf: true,
                },
                {
                    key: 'node3-2',
                    label: 'Node 3.2',
                    opened: false,
                    isLeaf: true,
                },
            ],
        },
    ]
);

function selectNode(node: TreeNode) {
    selectedKeys.value = [node.key];
    emit('node-click', selectedKeys.value);
}

defineOptions({ name: 'UIFileTree' });
</script>

<style lang="scss" scoped>
.ui-file-tree {
    ul.ui-file-tree-list {
        display: flex;
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

            font-size: 14px;
            color: #666;

            .ui-file-tree-list-anchor {
                aspect-ratio: 1;
                font-size: 1em;
                margin-top: 5px;
                color: #888;
            }

            .ui-file-tree-list-content {
                flex: 1;
                flex-direction: column;
                justify-content: flex-start;
                text-align: left;
                width: 100%;
            }

            .ui-file-tree-list-label {
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
