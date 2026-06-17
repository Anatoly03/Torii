<template>
    <div class="view-project">
        <span>Project View: {{ projectPath }}</span>
    </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';

const route = useRoute();
const projectPath = route.params.projectPath as string;
</script>
