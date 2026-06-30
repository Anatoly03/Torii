<template>
    <n-spin
        class="file-editor-image"
        :class="{ 'drag-over': isDragOver, 'has-image': imageBlob }"
        @dragenter.prevent="isDragOver = true"
        @dragleave.prevent="isDragOver = false"
        @drop="onImageDrop"
        :show="loading > 0"
    >
        <div v-if="imageBlob" class="image-preview">
            <img
                class="file-image-preview"
                :src="imageBlob"
                alt="Image Preview"
            />
            <button
                class="delete-btn"
                @click="removeImage"
                title="Delete image"
            >
                <NIcon size="16">
                    <CloseOutline />
                </NIcon>
            </button>
        </div>
        <div v-else class="image-placeholder">
            <NIcon size="32">
                <ImageOutline />
            </NIcon>
        </div>
    </n-spin>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { NIcon, NSpin } from 'naive-ui';
import { CloseOutline, ImageOutline } from '@vicons/ionicons5';

const props = defineProps<{
    directory: string | null;
    name: string | null;
    component: string;
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
const imageBlob = ref<string | null>(null);
const loading = ref(0);
const isDragOver = ref(false);

function createImageUrl(bytes: Uint8Array, mimeType = 'image/png'): string {
    const blob = new Blob([bytes], { type: mimeType });
    return URL.createObjectURL(blob);
}

function revokeImageUrl() {
    if (!imageBlob.value) return;
    URL.revokeObjectURL(imageBlob.value);
}

function refreshImageBlob() {
    revokeImageUrl();

    if (imageData.value) {
        imageBlob.value = createImageUrl(imageData.value);
    } else {
        imageBlob.value = null;
    }
}

async function loadFile() {
    try {
        const file = await invoke<Uint8Array>('get_record_component', {
            directory: props.directory,
            name: props.name,
            component: props.component,
        });

        imageData.value = file.byteLength ? file : null;
    } catch (e) {
        console.warn(e);
        imageData.value = null;
    }

    refreshImageBlob();
}

async function loadImageFromHTML(html: string) {
    console.trace('loadImageFromHTML', html);

    // This is an expensive operation.
    loading.value += 1;

    // Parse the HTML to see if the top-most element is an <img> tag,
    // then extract the src attribute
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, 'text/html');

    if (doc.body.firstElementChild?.tagName !== 'IMG') return;
    const element = doc.body.firstElementChild! as HTMLImageElement;

    console.log(element.src);

    // Fetch the image data from the src URL and convert it to a Uint8Array.
    // This should work for URLs but probably not for local files paths (cors).
    const response = await fetch(element.src);
    const blob = await response.blob();
    const arrayBuffer = await blob.arrayBuffer();

    imageData.value = new Uint8Array(arrayBuffer);
    console.log('Loaded image data:', imageData.value);

    refreshImageBlob();
    loading.value -= 1;
}

async function onImageDrop(event: DragEvent) {
    event.preventDefault();
    isDragOver.value = false;

    const dt = event.dataTransfer;
    if (!dt) return;

    // Check if files are dropped
    if (dt.files && dt.files.length > 0) {
        console.log('Files dropped:', dt.files);
    }

    // Check if text is dropped
    const text = dt.getData('text/plain');
    if (text) {
        console.log('Text dropped:', text);
    }

    // Handle HTML
    const html = dt.getData('text/html');
    if (html) {
        await loadImageFromHTML(html);
        return;
    }
}

async function removeImage() {
    if (!props.directory || !props.name) return;

    await invoke('remove_record_component', {
        directory: props.directory,
        name: props.name,
        component: props.component,
    });

    loadFile();
    emit('refresh');
}

onMounted(async () => {
    // Start loading after 100ms if the image does not load immediately.
    // The delay is to prevent spinner flickering.
    setTimeout(() => loading.value += 1, 100);

    await loadFile();
    loading.value -= 1;
});
onUnmounted(async () => {
    revokeImageUrl();
});
</script>

<style lang="scss" scoped>
.file-editor-image {
    position: relative;
    justify-content: center;
    align-items: center;
    margin: 16px;
    display: flex;
    flex-direction: column;
    min-width: 200px;
    max-width: 200px;
    min-height: 200px;
    max-height: 200px;
    aspect-ratio: 1 / 1;
    background-color: #fafafa;
    z-index: 10;
    border: 2px dashed #ccc;
    border-radius: 8px;
    overflow: hidden;

    // existing styles
    &.drag-over {
        outline: 2px solid #42b983;
        background-color: rgba(66, 185, 131, 0.1);
    }

    .image-preview {
        pointer-events: none;
    }

    .file-image-preview {
        max-width: 100%;
        max-height: 100%;
        object-fit: contain;
        display: block;
        pointer-events: none;
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

    .image-placeholder {
        height: 100%;
        color: #ccc;
        display: flex;
        align-items: center;
        justify-content: center;
    }
}
</style>
