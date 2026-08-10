<template>
    <router-view />
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { watchLocale } from './composables/watch-locale';
import { loadSettings } from './stores/settings.ts';

watchLocale();
loadSettings();

function preventDefault(e: { preventDefault: () => void }) {
    e.preventDefault();
}

onMounted(() => {
    document.addEventListener('dragover', preventDefault);
    document.addEventListener('drop', preventDefault);
});

onUnmounted(() => {
    document.removeEventListener('dragover', preventDefault);
    document.removeEventListener('drop', preventDefault);
});
</script>
