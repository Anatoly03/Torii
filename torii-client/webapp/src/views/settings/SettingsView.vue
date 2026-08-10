<template>
    <n-layout has-sider class="view-settings">
        <n-layout-sider bordered class="view-settings-side">
            <n-menu
                v-model:value="activeSetting"
                :options="menuOptions"
                @update:value="openSetting"
            />
        </n-layout-sider>
        <n-layout class="view-settings-content">
            <router-view />
        </n-layout>
    </n-layout>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { NLayout, NLayoutSider, NMenu } from 'naive-ui';
import { useI18n } from 'vue-i18n';

const i18n = useI18n();
const router = useRouter();
const route = useRoute();
const activeSetting = ref(route.name);
const menuOptions = computed(() => [
    {
        label: i18n.t('app.settings.general'),
        key: 'settings-general',
    },
    {
        label: i18n.t('app.settings.themes'),
        key: 'settings-theme',
    },
]);

function openSetting(key: string) {
    router.push({ name: key });
    // const option = menuOptions.find((opt) => opt.key === key);
    // if (!option) return;
    // router.push({ name: option.key });
}
</script>

<style lang="scss" scoped>
.view-settings {
    display: flex;
    flex-direction: row;
    height: 100%;

    .view-settings-content {
        display: flex;
        flex: 1;
        flex-direction: column;
        align-items: center;
        padding: 16px;
    }
}
</style>
