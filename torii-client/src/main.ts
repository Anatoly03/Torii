import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';
import App from './App.vue';
import router from './router';
import '@/styles/global.scss';

import en from './locales/en.json';
import jp from './locales/jp.json';
import zh from './locales/zh.json';

const i18n = createI18n({
    locale: 'en',
    fallbackLocale: 'en',
    messages: { en, jp, zh },
});

createApp(App).use(router).use(i18n).mount('#app');
