<template>
    <ul
        class="autocomplete-popup"
        :class="{ show: visible && suggestions.length > 0 }"
        :style="{ left: left + 'px', top: top + 'px' }"
    >
        <li
            v-for="record in suggestions"
            :key="record.label"
            class="autocomplete-item"
            :class="{ active: currentSelection?.label == record?.label }"
            @mouseenter="currentSelection = record"
            @mousedown.prevent="emit('select', record)"
            @keydown="emit('select', record)"
        >
            {{ record.label }}
        </li>
    </ul>
</template>

<script setup lang="ts">
import { EditorView } from 'prosemirror-view';
import { onMounted, onUnmounted, ref } from 'vue';
import type { SuggestionItem } from './autocomplete-extension';

const props = defineProps<{
    editorView: EditorView;
    suggestions: SuggestionItem[];
}>();

const emit = defineEmits<{
    (e: 'select', suggestion: SuggestionItem): void;
}>();

const currentSelection = ref<SuggestionItem | undefined>(props.suggestions[0]);
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

function onKeyDown(event: KeyboardEvent) {
    if (!visible.value || props.suggestions.length === 0) return;

    // Block default behaviour for these keys
    if (event.key === 'Enter' || event.key === 'Tab') {
        event.preventDefault();
        event.stopPropagation();
    }

    switch (event.key) {
        case 'ArrowDown':
        case 'ArrowUp':
            event.preventDefault();
            const currentIndex = props.suggestions.findIndex(
                (s) => s.label === currentSelection.value?.label
            );
            const nextIndex =
                event.key === 'ArrowDown'
                    ? (currentIndex + 1) % props.suggestions.length
                    : (currentIndex - 1 + props.suggestions.length) %
                      props.suggestions.length;
            currentSelection.value = props.suggestions[nextIndex];
            break;

        case 'Enter':
        case 'Tab':
            if (currentSelection.value) {
                emit('select', currentSelection.value);
            }
            break;
        case 'Escape':
            event.preventDefault();
            hide();
            break;
    }
}

onMounted(() => {
    const dom = props.editorView.dom;
    dom.addEventListener('focus', show);
    dom.addEventListener('blur', hide);
    dom.addEventListener('keydown', onKeyDown, { capture: true });
});

onUnmounted(() => {
    const dom = props.editorView.dom;
    dom.removeEventListener('focus', show);
    dom.removeEventListener('blur', hide);
    dom.removeEventListener('keydown', onKeyDown, { capture: true });
});

defineExpose({
    realign,
    show,
    hide,
});
</script>

<style lang="scss" scoped>
.autocomplete-popup {
    position: fixed;
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

    li.autocomplete-item {
        list-style: none;
        padding: 4px 12px;
        cursor: pointer;

        &:hover,
        &.active {
            background-color: #f0f0f0;
        }
    }
}
</style>
