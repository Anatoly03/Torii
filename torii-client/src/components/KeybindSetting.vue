<template>
    <div
        class="input-keybind-config"
        tabindex="0"
        @keydown="onKeyDown"
        @keyup="onKeyUp"
    >
        <span
            class="input-keybind-single-combination"
            v-for="(combination, index) in keybindValue"
            @click="discardCombination(combination)"
            :key="index"
        >
            <span
                class="input-keybind-single-key"
                v-for="(key, index) in combination"
                :key="key"
            >
                <NIcon v-if="index != 0" class="input-separator">
                    <Add />
                </NIcon>
                {{ renderKey(key) }}
            </span>
        </span>
        <span class="input-keybind-single-combination input-recording" v-if="recordedKeys.length">
            <span
                class="input-keybind-single-key"
                v-for="(key, index) in recordedKeys"
                :key="key"
            >
                <NIcon v-if="index != 0" class="input-separator">
                    <Add />
                </NIcon>
                {{ renderKey(key) }}
            </span>
        </span>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useKeybindsStore } from '../stores/keybinds.ts';
import { NIcon } from 'naive-ui';
import { Add } from '@vicons/ionicons5';

type KeybindAttributes = typeof useKeybindsStore extends () => infer R
    ? R
    : never;
type KeybindNames = keyof KeybindAttributes['$state'];

const props = defineProps<{
    name: KeybindNames;
}>();

const keybindsStore = useKeybindsStore();
const keybindValue = computed(() => keybindsStore[props.name]);

const recordedKeys = ref<string[]>([]);
const holdingKeys = ref<Set<string>>(new Set());

/**
 * On key down, add to recorded keys and holding keys if not already present.
 * @param event KeyboardEvent
 */
function onKeyDown(event: KeyboardEvent) {
    if (!recordedKeys.value.includes(event.key)) {
        recordedKeys.value.push(event.key.length === 1 ? event.key.toUpperCase() : event.key);
    }
    holdingKeys.value.add(event.key);
}

/**
 * On key up, remove from holding keys and update the keybind value if no keys are being held.
 * If no keys are being held, update the keybind value.
 */
function onKeyUp(event: KeyboardEvent) {
    holdingKeys.value.delete(event.key);

    if (holdingKeys.value.size === 0) {
        if (recordedKeys.value.length !== 0) {
            keybindsStore[props.name] = [...keybindsStore[props.name], recordedKeys.value];
        }

        recordedKeys.value = [];
    }
}

function discardCombination(combination: string[]) {
    keybindsStore[props.name] = keybindsStore[props.name].filter(
        (c) => c !== combination
    );
}

function renderKey(key: string) {
    switch (key) {
        case ' ':
            return 'Space';
        case 'Control':
            return 'Ctrl';
        case 'Meta':
            return 'Cmd';
        default:
            return key;
    }
}
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
    gap: 4px;

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

        .input-keybind-single-key {
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;
            gap: 4px;

            min-width: 24px;

            &:last-child {
                margin-right: 0;
            }
        }
    }
}
</style>
