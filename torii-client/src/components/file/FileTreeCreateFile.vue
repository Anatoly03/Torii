<template>
    <n-input
        placeholder="New File"
        size="small"
        :clearable="true"
        v-model:value="newFileName"
        @keydown="handleKeydown"
    />
</template>

<script setup lang="ts">
import { NInput } from 'naive-ui';
import { computed, defineModel } from 'vue';

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
