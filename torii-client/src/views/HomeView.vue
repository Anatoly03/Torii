<template>
    <div class="view-home">
        <div class="view-home-side">
            <div class="view-recent-projects">
                {{ $t('placeholder.noRecentProjects') }}
            </div>
            <LanguageSelect />
        </div>
        <div class="view-home-content">{{ $t('menu.home') }}</div>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { listRecentProjects } from '../composables/listRecentProjects';

import LanguageSelect from '@/components/LanguageSelect.vue';

const i18n = useI18n();
const recentProjects = computed(() => listRecentProjects());
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
        border-right: 1px solid #ccc;

        .view-recent-projects {
            margin-bottom: 16px;
            font-size: 14px;
            color: #666;
            flex: 1;
        }
    }
}
</style>
