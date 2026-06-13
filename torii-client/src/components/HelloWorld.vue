<template>
    <div class="hello">
        <h1>{{ msg }}</h1>
    </div>
</template>

<script setup lang="ts">
defineProps<{
    msg: string;
}>();
</script>
