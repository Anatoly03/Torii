/**
 * @file torii-client/src/stores/keybinds.ts
 * @brief Pinia Store for the Torii client application. This store manages the keybinds for the application.
 */

import { defineStore } from 'pinia';
import { ref, watch, onScopeDispose } from 'vue';
import { emit, listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Store } from '@tauri-apps/plugin-store';

const currentWindowId = getCurrentWindow().label;
let store: Store | null = null;

/**
 * Loads the keybinds from the local application storage.
 * @returns Keybinds store instance.
 */
export async function loadKeybinds() {
    if (!store) {
        store = await Store.load('keybinds.json');
    }
    return store;
}

/**
 * @brief Pinia Store for the Torii client application. This
 */
export const useKeybindsStore = defineStore('keybinds', () => {
    const unlisteners: (() => void)[] = [];

    /**
     * @brief Creates a reactive keybind with a default value and
     * sets up synchronization with the backend.
     */
    function createKeybind(name: string, defaultValue: string[][]) {
        const keybind = ref<string[][]>(defaultValue);

        let skipEmits = 0;

        // Load the initial value from the store if available.
        store?.get(name).then((value) => {
            if (value === undefined) return;
            if (!Array.isArray(value)) return;
            keybind.value = value as string[][];
        });

        // Listen for changes from the backend and update the setting.
        listen(`update:keybind:${name}`, (event: any) => {
            const source: string = event.payload.source;
            const value: string[][] = event.payload.value;

            // Ignore updates from the same window to prevent feedback loops.
            if (source === currentWindowId) return;
            if (keybind.value === value) return;

            // Prevent emitting an update back to the backend when we update the setting.
            skipEmits += 1;
            keybind.value = value;
        }).then((unlisten) => unlisteners.push(unlisten));

        // Watch for changes in the setting and emit an event to the backend.
        watch(keybind, (newValue) => {
            // Prevent emitting an update back to the backend when we update the setting
            // from an external source.
            if (skipEmits > 0) {
                skipEmits -= 1;
                return;
            }

            emit(`update:keybind:${name}`, {
                source: currentWindowId,
                value: newValue,
            });

            if (store) {
                store.set(name, newValue);
                store.save();
            }
        });

        return keybind;
    }

    // Cleanup listeners when store is destroyed (window closes)
    onScopeDispose(() => {
        unlisteners.forEach((fn) => fn());
    });

    return {
        textActionBold: createKeybind('textActionBold', [
            ['Cntrl', 'B'],
            ['Cmd', 'B'],
        ]),
        textActionItalic: createKeybind('textActionItalic', [
            ['Cntrl', 'I'],
            ['Cmd', 'I'],
        ]),
        textActionUnderline: createKeybind('textActionUnderline', [
            ['Cntrl', 'U'],
            ['Cmd', 'U'],
        ]),
    };
});
