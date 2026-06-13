<template>
    <nav>
        <router-link to="/">{{ $t('menu.home') }}</router-link> |
        <router-link to="/about">{{ $t('menu.about') }}</router-link>
    </nav>
    <router-view />
    <language-select />
</template>

<script setup lang="ts">
import LanguageSelect from './components/LanguageSelect.vue';
</script>
