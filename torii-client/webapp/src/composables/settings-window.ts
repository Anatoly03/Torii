import { ref } from 'vue';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

/**
 * @brief The instance of the settings window. There can only be one settings
 * window at any time.
 */
const settingsWindow = ref<WebviewWindow | null>(null);

/**
 * @brief Create a settings window if it does not exist or from an early hot
 * reload session.
 * @returns Reference to the settings window.
 */
export async function openSettingsWindow() {
    // Avoid opening multiple settings windows.
    if (settingsWindow.value) {
        settingsWindow.value.setFocus();
        return settingsWindow.value;
    }

    // On hot reload, set `settingsWindow` to current window because it is still open.
    if (import.meta.hot) {
        const existingWindow = await WebviewWindow.getByLabel('settings');
        if (existingWindow) {
            settingsWindow.value = existingWindow;
            settingsWindow.value.setFocus();
            return existingWindow;
        }
    }

    // Singleton: If the settings window does not exist yet, create it.
    settingsWindow.value = new WebviewWindow('settings', {
        url: '/settings',
        title: 'Settings',
        width: 860,
        height: 500,
        center: true,
        resizable: true,
        fullscreen: false,
    });

    settingsWindow.value.once('tauri://created', () => {
        console.log('Settings window created');
    });

    settingsWindow.value.once('tauri://close-requested', (_e) => {
        settingsWindow.value?.close();
        settingsWindow.value = null;
    });

    settingsWindow.value.once('tauri://error', (e) => {
        console.error('Failed to create settings window', e);
    });

    settingsWindow.value.once('tauri://error', (e) => {
        console.error('Failed to create settings window', e);
        settingsWindow.value = null; // Also clear on error
    });

    return settingsWindow.value;
}
