<template>
    <div class="home">
        <img alt="Vue logo" src="../assets/logo.png" />
        <HelloWorld :msg="msg" />
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import HelloWorld from '@/components/HelloWorld.vue';

const i18n = useI18n();
const msg = computed(() => i18n.t('app.welcome'));
</script>
