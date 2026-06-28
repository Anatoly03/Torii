<template>
    <n-input
        :bordered="false"
        class="file-tree-create-file"
        placeholder="New File"
        size="small"
        :clearable="true"
        v-model:value="newFileName"
        @blur="emit('cancel')"
        @click.stop
        @keydown="handleKeydown"
    />
</template>

<script setup lang="ts">
import { NInput } from 'naive-ui';
import { computed } from 'vue';

const newFileName = defineModel<string>();
const fileNameTrim = computed(() => newFileName.value?.trim() ?? '');
const emit = defineEmits<{
    (e: 'create', fileName: string): void;
    (e: 'cancel'): void;
}>();

function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === 'Escape') {
        if (e.key === 'Enter' && fileNameTrim.value !== '') {
            emit('create', fileNameTrim.value);
            return;
        }

        emit('cancel');
    }
}
</script>

<style lang="scss" scoped>
.file-tree-create-file {
    background-color: transparent !important;

    :deep(.n-input-wrapper) {
        background-color: transparent !important;
        padding: 0 !important;
        border: none !important; // already done
    }
    :deep(.n-input-el) {
        // the actual input element
        background-color: transparent !important;
        padding: 0 !important;
    }
    // Also maybe handle hover/focus states
    :deep(.n-input-wrapper:hover),
    :deep(.n-input-wrapper:focus-within) {
        background-color: transparent !important;
    }
}
</style>
