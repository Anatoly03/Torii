/**
 * @file torii-client/src/stores/settings.ts
 */

import { defineStore } from 'pinia';
import { ref, watch, onScopeDispose } from 'vue';
import { emit, listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Store } from '@tauri-apps/plugin-store';

const currentWindowId = getCurrentWindow().label;
let store: Store | null = null;

/**
 * Loads the settings from the local application storage.
 * @returns Settings store instance.
 */
export async function loadSettings() {
    if (!store) {
        store = await Store.load('settings.json');
    }
    return store;
}

/**
 * @brief Pinia Store for the Torii client application. This
 */
export const useSettingsStore = defineStore('settings', () => {
    const unlisteners: (() => void)[] = [];

    /**
     * @brief Creates a reactive setting with a default value and
     * sets up synchronization with the backend.
     */
    function createSetting<T>(name: string, defaultValue: T) {
        const setting = ref<T>(defaultValue);

        let skipEmits = 0;

        // Load the initial value from the store if available.
        store?.get(name).then((value) => {
            console.log(`Loaded setting ${name} from store:`, value);

            if (value === undefined) return;
            setting.value = value as T;

            console.log(`Setting ${name} initialized to:`, setting.value);
        });

        // Listen for changes from the backend and update the setting.
        listen(`update:setting:${name}`, (event: any) => {
            console.log(`Received update for setting ${name} from backend:`, event.payload);

            const source: string = event.payload.source;
            const value: T = event.payload.value;

            // Ignore updates from the same window to prevent feedback loops.
            if (source === currentWindowId) return;

            if (setting.value === value) return;

            // Prevent emitting an update back to the backend when we update the setting.
            skipEmits += 1;
            setting.value = value;
        }).then((unlisten) => unlisteners.push(unlisten));

        // Watch for changes in the setting and emit an event to the backend.
        watch(setting, (newValue) => {
            console.log(`Setting ${name} changed to:`, newValue, 'with skip emits:', skipEmits, 'and store:', store);

            // Prevent emitting an update back to the backend when we update the setting
            // from an external source.
            if (skipEmits > 0) {
                skipEmits -= 1;
                return;
            }

            emit(`update:setting:${name}`, {
                source: currentWindowId,
                value: newValue,
            });

            if (store) {
                store.set(name, newValue);
                store.save();
            }
        });

        return setting;
    }

    //
    // SETTINGS
    //
    /**
     * @brief Enables the word count in the footer of a project.
     */
    const enableWordCount = createSetting('enableWordCount', false);

    //
    // END SETTINGS
    //

    // Cleanup listeners when store is destroyed (window closes)
    onScopeDispose(() => {
        unlisteners.forEach((fn) => fn());
    });

    return { enableWordCount };
});
