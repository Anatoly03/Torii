<template>
    <div class="home">Welcome to the Launcher Page!</div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

const i18n = useI18n();
const msg = computed(() => i18n.t('app.welcome'));
</script>
