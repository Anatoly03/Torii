import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import { world } from '@torii/plugin-core';

import { createI18n } from 'vue-i18n';
import en from './locales/en.json';
import jp from './locales/jp.json';
import zh from './locales/zh.json';

console.log(world);

const i18n = createI18n({
    legacy: false,
    locale: 'en',
    fallbackLocale: 'en',
    messages: { en, jp, zh },
});

import { createPinia } from 'pinia';
const pinia = createPinia();

import { getCurrentWindow } from '@tauri-apps/api/window';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

const mainWindow = getCurrentWindow();
mainWindow.listen('tauri://close-requested', async (_event) => {
    const settings = await WebviewWindow.getByLabel('settings');

    if (settings) {
        await settings.close();
    }

    mainWindow.destroy();
});

import '@/styles/global.scss';

createApp(App).use(router).use(pinia).use(i18n).mount('#app');
