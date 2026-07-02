<template>
    <!-- <div class="view-settings-general"> -->
    <n-form label-placement="left" label-width="auto">
        <n-form-item label="Text Action Bold">
            <KeybindSetting name="textActionBold" />
        </n-form-item>
        <n-form-item label="Text Action Italic">
            <KeybindSetting name="textActionItalic" />
        </n-form-item>
        <n-form-item label="Text Action Underline">
            <KeybindSetting name="textActionUnderline" />
        </n-form-item>
    </n-form>
    <!-- </div> -->
</template>

<script setup lang="ts">
import LanguageSelect from '../../components/LanguageSelect.vue';
import { NForm, NFormItem, NSwitch } from 'naive-ui';
import { useSettingsStore } from '../../stores/settings.ts';
import KeybindSetting from '../../components/KeybindSetting.vue';

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
