import { ref } from 'vue';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

const settingsWindow = ref<WebviewWindow | null>(null);

export function openSettingsWindow() {
    // Avoid opening multiple settings windows.
    if (settingsWindow.value) {
        settingsWindow.value.setFocus();
        return;
    }

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

    settingsWindow.value.once('tauri://close-requested', (e) => {
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
}