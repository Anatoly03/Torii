<template>
    <!-- <div class="view-settings-general"> -->
    <n-form label-placement="left" label-width="auto">
        <n-form-item label="Language">
            <LanguageSelect />
        </n-form-item>
        <n-form-item :label="$t('app.settings.enableWordCount')">
            <n-switch v-model:value="settings.enableWordCount" />
        </n-form-item>
    </n-form>
    <!-- </div> -->
</template>

<script setup lang="ts">
import LanguageSelect from '../../components/LanguageSelect.vue';
import { NForm, NFormItem, NSwitch } from 'naive-ui';
import { useSettingsStore } from '../../stores/settings.ts';

const settings = useSettingsStore();
</script>

<style lang="scss" scoped>
.view-settings-general {
    display: flex;
    flex: 1;
    flex-direction: column;
    // align-items: center;
}
</style>
