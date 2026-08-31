<template>
    <select :value="modelValue" @change="onChange">
        <option
            v-for="option in options"
            :key="option.value"
            :value="option.value"
        >
            {{ option.label }}
        </option>
    </select>
</template>

<script lang="ts" setup>
const props = defineProps<{
    modelValue: string;
    options: { value: string; label: string }[];
}>();

const emit = defineEmits(['update:modelValue']);

function onChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    emit('update:modelValue', target.value);
}
</script>
