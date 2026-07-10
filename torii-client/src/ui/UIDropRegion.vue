<!-- This is a drag and drop listener region. It is a reusable component
    capable of handling drag and drop events. -->

<template>
    <div
        class="ui-drop-region"
        :class="{
            'drag-over': isDragOver,
            // 'drag-over-invalid': isUnimplementedDragover,
        }"
        :draggable="props.draggable"
        @dragstart="onDrag"
        @dragenter.prevent="isDragOver = true"
        @dragleave.prevent="isDragOver = false"
        @drop="onDrop"
    >
        <slot></slot>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

// const dropRegionEl = ref<HTMLElement | null>(null);
const isDragOver = ref(false);
// TODO implement ondragenter check if the file type would be handled
// const isUnimplementedDragover = ref(false);

const props = defineProps<{
    /**
     * Makes the drop region draggable. This is useful for in-app drag and drop
     * operations.
     *
     * # Example
     *
     * ```vue
     * <template>
     *   <ui-drop-region draggable @drag="drag => drag.dataTransfer.setData('text/plain', 'Hello World!')">
     *     Hello World!
     *   </ui-drop-region>
     * </template>
     * ```
     */
    draggable?: boolean;

    /**
     * Data types for dragging. This is an array of tuples, consisting of a MIME type and the corresponding
     * data.
     *
     * # Example
     *
     * ```vue
     * <template>
     *   <ui-drop-region draggable :drag-data="[ ['text/plain', 'Hello World!'] ]">
     *     Hello World!
     *   </ui-drop-region>
     * </template>
     * ```
     */
    dragData?: [string, string][];

    /**
     * Handles the drag event. Requires {@link draggable} to be set to true.
     *
     * # Example
     *
     * ```vue
     * <template>
     *   <ui-drop-region draggable @drag="drag => drag.dataTransfer.setData('text/plain', 'Hello World!')">
     *     Hello World!
     *   </ui-drop-region>
     * </template>
     * ```
     */
    onDrag?: (event: DragEvent) => void;

    /**
     * Handles the drop event. If no other drop handlers were invoked. This is
     * a general fallback handler for any drop event. This function contains the
     * original drag event argument.
     *
     * # Example
     *
     * ```vue
     * <template>
     *   <ui-drop-region @drop-url="url => src = url">
     *     <img v-if="src" :src="src" />
     *   </ui-drop-region>
     * </template>
     * ```
     */
    onDrop?: (event: DragEvent) => void;

    /**
     * Handles the drop event for "files" data dropped. This function contains a
     * list of File objects.
     */
    onDropFiles?: (files: File[]) => void;

    /**
     * Handles the drop event for "url" data dropped. This function contains a
     * remote URL argument. If you want to handle local file paths, use
     * `@drop-local-path`
     *
     * # Example
     *
     * ```vue
     * <template>
     *   <ui-drop-region @drop-url="url => src = url">
     *     <img v-if="src" :src="src" />
     *   </ui-drop-region>
     * </template>
     * ```
     */
    onDropUrl?: (url: string) => void;

    /**
     * Handles the drop event for "local file path" data dropped. This function
     * contains a local file path argument. If you want to handle remote URLs, use
     * `@drop-url`.
     *
     * # Example
     *
     * ```vue
     * <template>
     *   <ui-drop-region @drop-local-path="path => loadImage(path)">
     *     <img v-if="src" :src="src" />
     *   </ui-drop-region>
     * </template>
     * ```
     */
    onDropLocalPath?: (path: string) => void;

    /**
     * Handles the drop event for "html" data dropped. This function
     * contains an HTML string argument.
     *
     * This function is invoked after `@drop-url` and `@drop-local-path`
     * handlers are invoked. If the above events are defined and the HTML
     * looks like a "dropped path", this handler will not be invoked.
     *
     * # Example
     *
     * ```vue
     * <template>
     *   <ui-drop-region @drop-html="html => processDroppedHTML(html)">
     *     <div v-html="htmlString"></div>
     *   </ui-drop-region>
     * </template>
     * ```
     */
    onDropHtml?: (html: Element) => void;

    /**
     * Handles the drop event for "application/x-record-path" data dropped. This
     * function contains a record path.
     *
     * This function is invoked after `@drop-url` and `@drop-local-path`
     * handlers are invoked. If the above events are defined and the HTML
     * looks like a "dropped path", this handler will not be invoked.
     *
     * # Example
     *
     * ```vue
     * <template>
     *   <ui-drop-region @drop-application-record-path="path => loadRecord('./README')">
     *     README
     *   </ui-drop-region>
     * </template>
     * ```
     */
    onDropApplicationRecordPath?: (path: string) => void;

    // dropFiles?: (files: File[]) => void;
    // dropText?: (text: string) => void;
}>();

/**
 * Handles the drag event for the drag and drop region.
 */
function onDrag(event: DragEvent) {
    if (props.onDrag) {
        props.onDrag(event);
    }

    // If the user has defined drag data, set it on the event's dataTransfer object.
    setDragData(event);
}

/**
 *
 * @param event
 */
function setDragData(event: DragEvent) {
    if (!event.dataTransfer) return;
    if (!props.dragData) {
        event.dataTransfer.setData('text/html', event.target?.outerHTML ?? '');
        return;
    }

    for (const [type, data] of props.dragData) {
        event.dataTransfer!.setData(type, data);
    }
}

/**
 * Handles the drop event for the drag and drop region.
 * @param event - The drag event that triggered the drop.
 */
function onDrop(event: DragEvent) {
    event.preventDefault();
    isDragOver.value = false;
    // isUnimplementedDragover.value = false;

    const dt = event.dataTransfer;
    if (!dt) return;

    // Check if files are dropped
    if (dt.files && dt.files.length > 0) {
        const files: File[] = Array.from(dt.files);
        if (onDropFiles(files)) return;
    }

    // for security reasons, we cannot read `text/uri-list` because webview2 (tauri) blocks
    // data reads from external windows for this MIME
    // if (dt.types.includes('text/uri-list')) { ... }

    if (dt.types.includes('text/html')) {
        const html = dt.getData('text/html');
        if (onDropHtml(html)) return;
    }

    // TODO handle text/plain

    if (dt.types.includes('application/x-record-path')) {
        const recordPath = dt.getData('application/x-record-path');
        if (props.onDropApplicationRecordPath) {
            props.onDropApplicationRecordPath(recordPath);
            return;
        }
    }

    // Invoke general drop handler if no other handlers were invoked
    if (props.onDrop) {
        props.onDrop(event);
    }
}

/**
 * Handles the drop event for files dropped. This function should only be
 * invoked from {@link onDrop} for the {@link File} data type.
 * @param files - An array of {@link File} objects that were dropped.
 * @returns A boolean indicating whether the drop was handled successfully.
 * If it was not, other drop handlers may be invoked to handle the drop event.
 */
function onDropFiles(files: File[]): boolean {
    if (props.onDropFiles) {
        props.onDropFiles(files);
        return true;
    }
    return false;
}

/**
 * Handles the drop event for "html" data dropped. This function should
 * only be invoked from {@link onDrop} for 'text/html' data types.
 * @param html - The HTML string that was dropped.
 * @returns A boolean indicating whether the drop was handled successfully.
 * If it was not, other drop handlers may be invoked to handle the drop event.
 */
function onDropHtml(html: string): boolean {
    try {
        // Parse the HTML string into a document object and get the root node
        const parser = new DOMParser();
        const doc = parser.parseFromString(html, 'text/html');
        const root = doc.body.firstElementChild;
        if (!root) return false;

        // Test if html is some kind of recognized path reference (e.g. dragged
        // from file manager or browser).
        const rootTag = root.tagName.toLowerCase();
        switch (rootTag) {
            case 'img':
                if (onDropHtmlImage(root as HTMLImageElement)) return true;
                break;
            case 'a':
                if (onDropHtmlUrl(root as HTMLAnchorElement)) return true;
                break;
            default:
                return false;
        }

        // If we reach here, the dropped HTML was not recognized to be any path.
        if (props.onDropHtml) {
            props.onDropHtml(root as Element);
            return true;
        }
    } catch (e) {
        console.error('Failed to handle dropped HTML:', e);
        return true; // assume success and "panic" other handlers for errors
    }

    return false;
}

/**
 * Handles the drop event for "html" data dropped, with an <img> root.
 * This function should only be invoked from {@link onDropHtml} when the
 * parsed node is an <img> element.
 * @param element - The <img> element that was dropped.
 * @returns A boolean indicating whether the drop was handled successfully.
 * If it was not, other drop handlers may be invoked to handle the drop event.
 */
function onDropHtmlImage(element: HTMLImageElement): boolean {
    const src = element.src;
    if (!src) return false;

    // Skip blobs (in the future we can add new handlers for this)
    if (src.startsWith('data:') || src.startsWith('blob:')) {
        return false;
    }

    // Invoke url handler
    if (props.onDropUrl) {
        props.onDropUrl(src);
        return true;
    }

    return false;
}

/**
 * Handles the drop event for "html" data dropped, with an <a> root.
 * This function should only be invoked from {@link onDropHtml} when the
 * parsed node is an <a> element.
 * @param element - The <a> element that was dropped.
 * @returns A boolean indicating whether the drop was handled successfully.
 * If it was not, other drop handlers may be invoked to handle the drop event.
 */
function onDropHtmlUrl(element: HTMLAnchorElement): boolean {
    // Invoke url handler if "href" is present
    if (element.href && props.onDropUrl) {
        props.onDropUrl(element.href);
        return true;
    }

    const text = element.innerText;

    // Invoke local file handler if "file://" is present in the text. This is
    // how the Thunar file manager provides dragged files.
    if (text.startsWith('file://')) {
        const path = decodeURIComponent(text.substring('file://'.length));
        if (props.onDropLocalPath) {
            props.onDropLocalPath(path);
            return true;
        }
    }

    // Invoke the url handler if the text is a valid URL.
    if (text.startsWith('http://') || text.startsWith('https://')) {
        if (props.onDropUrl) {
            props.onDropUrl(text);
            return true;
        }
    }

    return false;
}
</script>

<style lang="scss" scoped>
.ui-drop-region {
    &.drag-over:not(.drag-over-invalid) {
        outline: 2px solid #42b983;
        background-color: rgba(66, 185, 131, 0.1);
        cursor: default;
    }
}
</style>
