<template>
    <div class="view-home">
        <div class="view-home-side">
            <RecentProjectsView />
            <div class="view-home-quick-settings">
                <LanguageSelect class="language-select" />
                <button
                    class="view-home-settings-button"
                    @click="openSettingsWindow()"
                >
                    <Icon><SettingsOutline /></Icon>
                </button>
            </div>
        </div>
        <div class="view-home-content">
            {{ $t('app.menu.home') }}
            <small class="version">
                {{ $t('app.version.verbose', { version }) }}
            </small>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getVersion } from '@tauri-apps/api/app';
import { Icon } from '@vicons/utils';
import { SettingsOutline } from '@vicons/ionicons5';
import LanguageSelect from '@/components/LanguageSelect.vue';
import RecentProjectsView from './RecentProjectsView.vue';
import { openSettingsWindow } from '../composables/settings-window.ts';

const version = ref('Invalid Version');

onMounted(async () => {
    try {
        version.value = await getVersion();
    } catch (error) {
        console.error('Failed to get app version:', error);
    }
});
</script>

<style lang="scss" scoped>
.view-home {
    display: flex;
    flex-direction: row;
    height: 100%;

    .view-home-side {
        display: flex;
        flex-direction: column;
        width: 200px;
        padding: 16px;
        gap: 8px;
        border-right: 1px solid #ccc;

        .view-home-quick-settings {
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 8px;
            min-height: 2em;

            .language-select {
                flex: 1;
            }
        }
    }

    .view-home-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        flex: 1;
    }
}
</style>
