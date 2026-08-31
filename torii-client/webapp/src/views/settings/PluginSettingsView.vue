<template>
    <div class="view-settings-plugins">
        <b>Active Plugins</b>
        <div class="active-plugin-item" v-for="(plugin, idx) in activePlugins" :key="idx">
            <span class="title">{{ plugin.name }}</span>
            <span class="version">{{ plugin.version }}</span>
        </div>
    </div>
</template>

<script setup lang="ts">
import { usePlugins } from '@/plugins/loader';
import { computed } from 'vue';

const pluginManager = usePlugins();

const activePlugins = computed(() => {
    const results: any[] = [];
    const pluginIds = pluginManager.plugins().value;

    for (const pid of pluginIds) {
        const plugin = pluginManager.getPlugin(pid);
        if (!plugin) continue;

        results.push({
            name: pid,
            version: plugin.version,
        });
    }

    return results;
});
</script>

<style lang="scss" scoped>
.view-settings-plugins {
    display: flex;
    flex: 1;
    flex-direction: column;
    align-items: center;
    width: 100%;
    gap: 16px;

    .active-plugin-item {
        display: flex;
        width: 100%;
        flex-direction: row;
        justify-content: space-between;
        gap: 16px;

        .version {
            color: gray;
        }
    }
}
</style>
