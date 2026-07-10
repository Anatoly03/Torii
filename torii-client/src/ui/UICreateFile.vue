<template>
    <div class="file-tree-create-file">
        <button
            v-if="!creatingNewFile"
            @click="creatingNewFile = true"
            class="new-file-btn"
        >
            <NIcon><AddOutline /></NIcon>
            {{ $t('app.project.newFile') }}
        </button>
        <n-input
            v-if="creatingNewFile"
            :bordered="false"
            placeholder="New File"
            size="small"
            :clearable="true"
            v-model:value="newFileName"
            @blur="creatingNewFile = false"
            @click.stop
            @keydown="handleKeydown"
        />
    </div>
</template>

<script setup lang="ts">
import { NInput } from 'naive-ui';
import { computed, ref } from 'vue';

const creatingNewFile = ref(false);
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
    display: flex;
    flex-direction: column;

    width: 100%;
    height: 32px;
    text-align: left;
    padding-left: 22px;
    box-sizing: border-box;
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

.new-file-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    border: 1px dashed #ccc;
    border-radius: 4px;
    background: transparent;
    color: #666;
    cursor: pointer;
    transition: all 0.2s;
    box-sizing: border-box;
    width: 100%;

    &:hover {
        border-color: #42b983;
        color: #42b983;
        background: #f0faf5;
    }
}

</style>
