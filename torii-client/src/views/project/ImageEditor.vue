<template>
    <div class="file-editor-image" v-if="imageSrc">
        <img class="file-image-preview" :src="imageSrc" alt="Image Preview" />
        <button class="delete-btn" @click="removeImage" title="Delete image">
            <NIcon size="16">
                <CloseOutline />
            </NIcon>
        </button>
    </div>
    <div v-else class="file-editor-image">
        <div class="file-image-placeholder">
            <NIcon size="32">
                <ImageOutline />
            </NIcon>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { NIcon } from 'naive-ui';
import { CloseOutline, ImageOutline } from '@vicons/ionicons5';

const props = defineProps<{
    directory: string | null;
    name: string | null;
}>();

const emit = defineEmits<{
    (e: 'refresh'): void;
}>();

watch(
    () => [props.directory, props.name],
    () => {
        loadFile();
    }
);

const imageData = ref<Uint8Array | null>(null);
const imageSrc = computed(() => {
    revokeImageUrl(); // Revoke the previous object URL to avoid memory leaks
    return imageData.value ? createImageUrl(imageData.value) : '';
});

async function loadFile() {
    try {
        imageData.value = await invoke<Uint8Array>('get_record_component', {
            directory: props.directory,
            name: props.name,
            component: 'image',
        });
    } catch(e) {
        console.warn(e);
        imageData.value = null;
    }
}

async function removeImage() {
    if (!props.directory || !props.name) return;

    await invoke('remove_record_component', {
        directory: props.directory,
        name: props.name,
        component: 'image',
    });

    imageData.value = null;
    emit('refresh');
}

function createImageUrl(bytes: Uint8Array, mimeType = 'image/png'): string {
    const blob = new Blob([bytes], { type: mimeType });
    return URL.createObjectURL(blob);
}

function revokeImageUrl() {
    if (imageSrc.value) {
        URL.revokeObjectURL(imageSrc.value);
    }
}

onMounted(loadFile);
onUnmounted(revokeImageUrl);
</script>

<style lang="scss" scoped>
.file-editor-image {
    position: relative;
    justify-content: center;
    align-items: center;
    padding: 16px;
    display: inline-block;

    .file-image-preview {
        max-height: 200px;
        object-fit: contain;
        display: block;
    }

    .delete-btn {
        position: absolute;
        top: 4px;
        right: 4px;
        width: 28px;
        height: 28px;
        border: none;
        border-radius: 5px;
        background: rgba(0, 0, 0, 0.6);
        color: #fff;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        opacity: 0;
        transition: opacity 0.2s ease, background 0.2s ease;
        padding: 0;

        &:hover {
            background: rgba(255, 44, 44, 0.8);
        }

        &:focus-visible {
            outline: 2px solid #fff;
            outline-offset: 2px;
        }
    }

    &:hover .delete-btn {
        opacity: 1;
    }

    .file-image-placeholder {
        width: 200px;
        height: 200px;
        border: 2px dashed #ccc;
        border-radius: 8px;
        color: #ccc;
        display: flex;
        align-items: center;
        justify-content: center;
    }
}
</style>
