<template>
    <div class="input-keybind-config" tabindex="0">
        <span class="input-keybind-single-combination" v-for="combination in keybindValue" :key="combination.join('-')">
            <span class="input-keybind-single-key" v-for="key in combination" :key="key">
                {{ key }}
            </span>
        </span>
    </div>
</template>

<script setup lang="ts">
import { useKeybindsStore } from '../stores/keybinds.ts';

/**
 * @brief Type for the names of the keybinds in the store.
 */
type KeybindAttributes = typeof useKeybindsStore extends () => infer R
    ? R
    : never;
type KeybindNames = keyof (KeybindAttributes['$state']);

const props = defineProps<{
    name: KeybindNames;
}>();

const keybindsStore = useKeybindsStore();
const keybindValue = keybindsStore[props.name];
</script>

<style lang="scss" scoped>
.input-keybind-config {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;

    // background-color: #eee;
    // padding: 4px 8px;

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
