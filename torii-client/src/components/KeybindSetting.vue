<template>
    <div class="input-keybind-config" tabindex="0">
        <span
            class="input-keybind-single-combination"
            v-for="(combination, index) in keybindValue"
            :key="index"
        >
            <span
                class="input-keybind-single-key"
                v-for="key in combination"
                :key="key"
            >
                {{ key }}
            </span>
        </span>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useKeybindsStore } from '../stores/keybinds.ts';

type KeybindAttributes = typeof useKeybindsStore extends () => infer R
    ? R
    : never;
type KeybindNames = keyof KeybindAttributes['$state'];

const props = defineProps<{
    name: KeybindNames;
}>();

const keybindsStore = useKeybindsStore();
const keybindValue = computed(() => keybindsStore[props.name]);

const isRecording = ref(false);
const recordedKeys = ref<string[]>([]);
</script>

<style lang="scss" scoped>
.input-keybind-config {
    display: flex;
    flex: 1;
    flex-direction: row;
    align-items: center;
    justify-content: start;
    width: 100%;
    min-width: 200px;
    height: 100%;

    background-color: #eee;
    padding: 4px 8px;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;

    &:focus {
        outline: 2px solid #42b983;
        background-color: #e0f0e8;
    }

    .input-keybind-single-combination {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        gap: 4px;

        .input-keybind-single-key {
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;

            min-width: 24px;

            background-color: #ccc;
            border-radius: 4px;
            padding: 0 4px;

            border-bottom: 1px solid #aaa;
            border-right: 1px solid #aaa;

            &:hover {
                background-color: #d2d2d2;
                border-bottom: 1px solid #aaa;
                border-right: 1px solid #aaa;
            }

            &:last-child {
                margin-right: 0;
            }
        }
    }
}
</style>
