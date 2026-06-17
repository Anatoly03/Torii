<template>
    <!-- <div class="view-settings-general"> -->
        <n-form label-placement="left" label-width="auto">
            <n-form-item label="Language">
                <LanguageSelect />
            </n-form-item>
        </n-form>
    <!-- </div> -->
</template>

<script setup lang="ts">
import LanguageSelect from '../../components/LanguageSelect.vue';
import { NForm, NFormItem } from 'naive-ui';
</script>

<style lang="scss" scoped>
.view-settings-general {
    display: flex;
    flex: 1;
    flex-direction: column;
    // align-items: center;
}
</style>
