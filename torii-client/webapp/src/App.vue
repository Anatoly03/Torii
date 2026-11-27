<template>
    <view-header />
    <main class="view-application-content">
        <router-view />
    </main>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { watchLocale } from './composables/watch-locale.ts';
import { loadSettings } from './stores/settings.ts';
import ViewHeader from './components/ViewHeader.vue';

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

<style lang="scss" scoped>
.view-application-content {
    flex: 1;
}
</style>