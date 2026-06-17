/**
 * @file locale.ts
 * @brief Binds the window-local locale reference to cross-window reactivity.
 */

import { watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { emit, listen } from '@tauri-apps/api/event';

/**
 * The name of the event emitted by the main process when the locale is updated.
 */
const LOCALE_CHANGE_EVENT = 'i18n-updated';

/**
 * Serves the locale watcher.
 */
export function watchLocale() {
    const { locale } = useI18n({ useScope: 'global' });

    // Listen for locale changes from other windows
    // const unlistenPromise =
    listen<string>(LOCALE_CHANGE_EVENT, (event) => {
        if (event.payload !== locale.value) {
            locale.value = event.payload;
        }
    });

    // Watch for window-local locale changes and emit an event to other windows
    // when it is updated.
    watch(locale, (newLocale, oldLocale) => {
        if (newLocale === oldLocale) return;
        emit(LOCALE_CHANGE_EVENT, newLocale);
    });
}
