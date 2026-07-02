/**
 * @file torii-client/src/stores/settings.ts
 */

import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { emit, listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';

const currentWindowId = getCurrentWindow().label;

/**
 * @brief Pinia Store for the Torii client application. This
 */
export const useSettingsStore = defineStore('settings', () => {
    /**
     * @brief Creates a reactive setting with a default value and
     * sets up synchronization with the backend.
     */
    function createSetting<T>(name: string, defaultValue: T) {
        const setting = ref<T>(defaultValue);
        let skipEmits = 0;

        // Listen for changes from the backend and update the setting.
        listen(`update:setting:${name}`, (event: any) => {
            const source: string = event.payload.source;
            const value: T = event.payload.value;

            // Ignore updates from the same window to prevent feedback loops.
            if (source === currentWindowId) return;

            if (setting.value === value) return;

            // Prevent emitting an update back to the backend when we update the setting.
            skipEmits += 1;
            setting.value = value;
        });

        // Watch for changes in the setting and emit an event to the backend.
        watch(setting, (newValue) => {
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
        });

        return setting;
    }

    //
    // SETTINGS
    //

    /**
     * @brief Enables the word count in the footer of a project.
     */
    const enableWordCount = createSetting('enableWordCount', true);

    return { enableWordCount };
});
