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

<style>
#app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
}

nav {
    padding: 30px;
}

nav a {
    font-weight: bold;
    color: #2c3e50;
}

nav a.router-link-exact-active {
    color: #42b983;
}
</style>
