<template>
    <router-view />
</template>

<script setup lang="ts">
import { watchLocale } from './composables/watch-locale';

watchLocale();
</script>
