<template>
    <div class="view-home">
        <div class="view-home-side">
            <div class="view-recent-projects">
                {{ $t('placeholder.noRecentProjects') }}
            </div>
            <div class="view-home-quick-settings">
                <LanguageSelect class="language-select" />
                <button class="view-home-settings-button" disabled>
                    <Icon><SettingsOutline /></Icon>
                </button>
            </div>
        </div>
        <div class="view-home-content">
            {{ $t('menu.home') }}
            <small class="version">
                {{ $t('version.verbose', { version }) }}
            </small>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { listRecentProjects } from '../composables/listRecentProjects';
import { getVersion } from '@tauri-apps/api/app';

import { Icon } from '@vicons/utils';
import { SettingsOutline } from '@vicons/ionicons5';
import LanguageSelect from '@/components/LanguageSelect.vue';

const i18n = useI18n();
const recentProjects = computed(() => listRecentProjects());
const version = ref(i18n.t('version.unknown'));

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

        .view-recent-projects {
            margin-bottom: 16px;
            font-size: 14px;
            color: #666;
            flex: 1;
        }

        .view-home-quick-settings {
            display: flex;
            flex-direction: row;
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
