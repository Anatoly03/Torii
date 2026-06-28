<template>
    <div class="file-editor-image">
        <img class="file-image-preview" :src="imageSrc" alt="Image Preview" />
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps<{
    directory: string | null;
    name: string | null;
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
    imageData.value = await invoke<Uint8Array>('get_record_component', {
        directory: props.directory,
        name: props.name,
        component: 'image',
    });
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
    justify-content: center;
    align-items: center;
    padding: 16px;

    .file-image-preview {
        max-height: 200px;
        object-fit: contain;
    }
}
</style>
