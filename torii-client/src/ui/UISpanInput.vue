<!-- The UI Span input is a label component which can be edited. -->

<template>
    <span
        ref="spanEl"
        class="label"
        :class="{ 'is-editing': isEditing }"
        :contenteditable="isEditing"
        @dblclick="onDoubleClick()"
        @keydown.enter.prevent="onSubmit($event)"
        @blur="onCancel()"
        @focusout="onCancel()"
    >
        {{ editValue }}
    </span>
</template>

<script setup lang="ts">
import { nextTick, ref } from 'vue';

const props = defineProps<{
    modelValue: string;
}>();

const spanEl = ref<HTMLElement | null>(null);
const editValue = ref(props.modelValue);

const emit = defineEmits<{
    (e: 'submit', value: string): void;
    (e: 'update:modelValue', value: string): void;
}>();

const isEditing = defineModel<boolean>('isEditing', {
    default: () => false,
});

function onDoubleClick() {
    isEditing.value = true;
    nextTick(() => {
        spanEl.value?.focus();
    });
}

function onCancel() {
    isEditing.value = false;
    editValue.value = props.modelValue;
    spanEl.value!.innerText = props.modelValue;
}

function onSubmit(event: Event) {
    isEditing.value = false;
    const newValue = (event.target as HTMLElement).innerText;
    if (newValue === props.modelValue) {
        return;
    }

    editValue.value = newValue;
    emit('update:modelValue', newValue);
    emit('submit', newValue);
}
</script>

<style lang="scss" scoped>
.label {
    position: relative;
    border-radius: 2px;

    &.is-editing {
        outline: 2px solid #42b983;   // or whichever colour
        z-index: 100;                 // bring above others
    }
}
</style>
