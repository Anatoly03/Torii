/**
 * @file torii-client/src/stores/settings.ts
 */

import { defineStore } from 'pinia';
import { ref } from 'vue';

/**
 * @brief Pinia Store for the Torii client application. This
 */
export const useSettingsStore = defineStore('settings', () => {
    /**
     * @brief Enables the word count in the footer of a project.
     */
    const enableWordCount = ref(true);

    return { enableWordCount };
});
