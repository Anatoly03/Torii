<template>
    <div class="view-project">
        <div class="view-project-side">
            <FileTree
                :root="projectPath"
                @update:current-file="currentFile = $event"
            />
            <div class="view-project-quick-settings">
                <button
                    class="view-project-return-to-menu"
                    @click="$router.push('/')"
                >
                    {{ $t('app.project.close') }}
                </button>
            </div>
        </div>
        <div class="view-project-content-placeholder" v-if="!currentFile">
            Project View: {{ projectPath }}<br />
        </div>
        <editor-content
            class="view-project-content"
            v-else
            :editor="editor"
        />
    </div>
</template>

<script setup lang="ts">
import { onUnmounted, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import FileTree from '../../components/file/FileTree.vue';
import { invoke } from '@tauri-apps/api/core';
import { Editor, EditorContent } from '@tiptap/vue-3';
import { Markdown } from '@tiptap/markdown';
import StarterKit from '@tiptap/starter-kit';

const route = useRoute();
const router = useRouter();
const projectPath = route.params.projectPath as string;
const currentFile = ref<string | null>(null);
const ignoreFirstSave = ref(true);
const editor = new Editor({
    extensions: [Markdown, StarterKit],
    content: "<p>I'm running Tiptap with Vue.js. 🎉</p>",
    onUpdate: saveFile,
});

if (!projectPath) {
    // If no project path is provided, redirect to the home page
    router.push('/');
}

watch(currentFile, async (path) => {
    if (!path) return;

    // Load the file content when a new file is selected
    let content = await invoke<string>('get_markdown_file', { path });
    console.debug('load', content);

    ignoreFirstSave.value = true;
    editor.commands.setContent(content, { contentType: 'markdown' });
});

async function saveFile() {
    if (!currentFile.value) return;
    if (ignoreFirstSave.value) {
        ignoreFirstSave.value = false;
        return;
    }

    // Get markdown content from the editor
    const content = editor.getMarkdown();
    console.debug('save', content);

    // Save the file content whenever it changes
    await invoke('save_markdown_file', {
        path: currentFile.value,
        content,
    });
}

onUnmounted(() => {
    editor.destroy();
});
</script>

<style lang="scss" scoped>
.view-project {
    display: flex;
    flex-direction: row;
    height: 100%;

    .view-project-side {
        display: flex;
        flex-direction: column;
        width: 200px;
        padding: 16px;
        gap: 8px;
        border-right: 1px solid #ccc;

        .view-project-quick-settings {
            display: flex;
            flex-direction: row;
            gap: 8px;
            min-height: 2em;

            button {
                flex: 1;
            }
        }
    }

    .view-project-content-placeholder {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        flex: 1;
    }

    .view-project-content {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        justify-content: flex-start;
        flex: 1;
        margin: 4px;
        text-align: left;

        p {
            margin: 0;
        }

        :deep(.ProseMirror) {
            max-height: 100%;
            outline: none;
            box-sizing: border-box;

            padding: 0 16px;
        }
    }
}
</style>
