<template>
    <ul
        class="autocomplete-popup"
        :class="{ show: visible }"
        :style="{ left: left + 'px', top: top + 'px' }"
    >
        <li
            v-for="record in suggestions"
            :key="record.label"
            @click="emit('select', record)"
            @keydown="emit('select', record)"
        >
            {{ record.label }}
        </li>
    </ul>
</template>

<script setup lang="ts">
import { EditorView } from 'prosemirror-view';
import { onMounted, ref } from 'vue';
import type { SuggestionItem } from './autocomplete-extension';

const props = defineProps<{
    editorView: EditorView;
    suggestions: SuggestionItem[];
}>();

const emit = defineEmits<{
    (e: 'select', suggestion: SuggestionItem): void;
}>();

const visible = ref(false);
const left = ref(0);
const top = ref(0);

function realign() {
    const cursor = props.editorView.state.selection.from;
    const coords = props.editorView.coordsAtPos(cursor);
    left.value = coords.left;
    top.value = coords.top + 10; // below cursor line
}

function show() {
    realign();
    visible.value = true;
}

async function hide() {
    await new Promise((resolve) => setTimeout(resolve, 100)); // wait for click event to propagate
    visible.value = false;
}

defineExpose({
    realign,
    show,
    hide,
});
</script>

<style lang="scss" scoped>
.autocomplete-popup {
    position: absolute;
    display: none;
    flex-direction: column;
    min-width: 128px;
    padding: 4px 0;

    background-color: #fff;
    border: 1px solid #ccc;
    border-radius: 4px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
    z-index: 1000;

    &.show {
        display: flex;
    }

    li {
        list-style: none;
        padding: 4px 8px;
        cursor: pointer;

        &:hover {
            background-color: #f0f0f0;
        }
    }
}
</style>
