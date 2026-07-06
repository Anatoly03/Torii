<template>
    <UIDropRegion
        class="file-editor-image"
        @drop-url="loadImageFromURL"
        @drop-local-path="loadImageFromFilePath"
        @drop-files="loadImageFromFiles"
        @click="onImageClick"
    >
        <n-spin :show="loading > 0">
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
            <div
                v-else
                class="image-placeholder"
                :class="{
                    ['image-placeholder-' + props.placeholderAnchor]:
                        props.placeholderAnchor !== undefined,
                }"
            >
                <NIcon size="32">
                    <ImageOutline />
                </NIcon>
                <span v-if="props.placeholderText">{{
                    props.placeholderText
                }}</span>
            </div>
        </n-spin>
    </UIDropRegion>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open as openFileDialog } from '@tauri-apps/plugin-dialog';
import { NIcon, NSpin } from 'naive-ui';
import { CloseOutline, ImageOutline } from '@vicons/ionicons5';
import { Record } from 'types';
import UIDropRegion from '../../ui/UIDropRegion.vue';

const props = defineProps<{
    record: Record;
    component: string;
    placeholderText?: string;
    placeholderAnchor: 'top' | 'bottom' | 'left' | 'right' | 'center';
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
            record: props.record,
            component: props.component,
        });

        imageData.value = file.byteLength ? file : null;
    } catch (e) {
        console.warn(e);
        imageData.value = null;
    }

    refreshImageBlob();
}

/**
 * Loads image from a "File" object.
 */
async function loadImageFromFiles(files: File[]) {
    if (!files || files.length === 0) return;
    const file = files[0];

    // This is an expensive operation. Start loading after 100ms
    // if the image does not load immediately.
    setTimeout(() => (loading.value += 1), 100);

    // Read as ArrayBuffer (binary)
    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    console.log(file);
    console.log(arrayBuffer);

    // Send to Rust via invoke (base64)
    const base64 = btoa(
        uint8Array.reduce((data, byte) => data + String.fromCharCode(byte), '')
    );

    await invoke('save_record_component', {
        record: props.record,
        component: 'image',
        content: base64,
        contentType: file.type,
    });

    // Refresh the image data after saving
    await loadFile();

    loading.value -= 1;
}

/**
 *
 * @param url URL of the image to load.
 */
async function loadImageFromURL(url: string) {
    // This is an expensive operation. Start loading after 100ms
    // if the image does not load immediately.
    setTimeout(() => (loading.value += 1), 100);

    // Fetch the image data from the src URL and convert it to a Uint8Array.
    // This should work for URLs but probably not for local files paths (cors).
    const response = await fetch(url);
    const blob = await response.blob();
    const arrayBuffer = await blob.arrayBuffer();
    const byteLength = arrayBuffer.byteLength;
    const uint8Array = new Uint8Array(arrayBuffer);

    imageData.value = uint8Array;
    refreshImageBlob();

    console.debug(`Loaded ${byteLength} bytes from ${url}`);

    // Convert the image data to a base64 string for saving
    // which is required for mime type "image"
    const base64 = btoa(
        uint8Array.reduce((data, byte) => data + String.fromCharCode(byte), '')
    );

    // Save the image data to the backend
    await invoke('save_record_component', {
        record: props.record,
        component: props.component,
        content: base64,
        contentType: 'image/png',
    });

    // Refresh the image data after saving
    await loadFile();

    loading.value -= 1;
}

/**
 *
 * @param path Local file path
 */
async function loadImageFromFilePath(source: string) {
    // Save the image data from a local file path. This is a special case for local files.
    await invoke('save_record_component_from_local_file', {
        record: props.record,
        component: props.component,
        source,
    });

    // Refresh the image data after saving
    await loadFile();
}

/**
 * If image was clicked and there is no image currently, open file
 * dialog and select image.
 */
async function onImageClick(_event: MouseEvent) {
    const path = await openFileDialog();
    if (!path) return;

    // Load the image from the selected file path using the copy
    // command.
    await invoke('save_record_component_from_local_file', {
        record: props.record,
        component: props.component,
        source: path,
    });

    // Refresh the image data after saving
    await loadFile();
}

async function removeImage() {
    if (!props.directory || !props.name) return;

    await invoke('remove_record_component', {
        record: props.record,
        component: props.component,
    });

    loadFile();
    emit('refresh');
}

onMounted(async () => {
    // Start loading after 100ms if the image does not load immediately.
    // The delay is to prevent spinner flickering.
    setTimeout(() => (loading.value += 1), 100);

    await loadFile();
    loading.value -= 1;
});

onUnmounted(async () => {
    revokeImageUrl();
});
</script>

<style lang="scss" scoped>
.file-editor-image {
    justify-content: center;
    align-items: center;
    display: flex;
    flex-direction: column;
    min-width: 200px;
    aspect-ratio: 1 / 1;
    overflow: hidden;

    &:not(.has-image) {
        cursor: pointer;
    }

    // existing styles
    &.drag-over {
        outline: 2px solid #42b983;
        background-color: rgba(66, 185, 131, 0.1);
        cursor: default;
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

    &:not(.has-image) {
        &:has(.image-placeholder-left) {
            align-items: flex-start;

            .image-placeholder {
                padding-left: 16px;
            }
        }

        &:has(.image-placeholder-right) {
            align-items: flex-end;

            .image-placeholder {
                padding-right: 16px;
            }
        }

        &:has(.image-placeholder-top) {
            justify-content: flex-start;

            .image-placeholder {
                padding-top: 16px;
            }
        }

        &:has(.image-placeholder-bottom) {
            justify-content: flex-end;

            .image-placeholder {
                padding-bottom: 16px;
            }
        }
    }

    .image-placeholder {
        height: 100%;
        color: #ccc;
        display: flex;
        gap: 8px;
        align-items: end;
        justify-content: center;
        font-size: 18px;
    }
}
</style>
