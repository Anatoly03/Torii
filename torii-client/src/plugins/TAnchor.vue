<template>
    <div
        class="torii-anchor"
        :class="{
            'torii-anchor-vertical': vertical,
            'torii-anchor-horizontal': horizontal,
        }"
    >
        <div v-for="(Comp, index) in renderedComponents" :key="index" class="torii-anchor-item">
            <component :is="Comp" v-if="Comp" />
        </div>
    </div>
</template>

<script setup lang="ts">
import { type Component, computed, defineAsyncComponent } from 'vue';
import { usePlugins } from './loader';

const props = defineProps<{
    id: string;
    vertical?: boolean;
    horizontal?: boolean;
}>();

const pluginManager = usePlugins();

// Helper that normalises any result from renderAnchor into a Component
function normalizeComponent(raw: unknown): Component | undefined {
    if (!raw) return undefined;

    // Case 1: It's a Promise
    if (raw instanceof Promise) {
        return defineAsyncComponent(() =>
            raw.then((resolved) => {
                // If resolved is an ES module with a default export, extract it
                if (resolved && typeof resolved === 'object' && 'default' in resolved) {
                    return resolved.default as Component;
                }
                // Otherwise assume it's a component directly
                return resolved as Component;
            })
        );
    }

    // Case 2: It's already an ES module object (sync import)
    if (typeof raw === 'object' && raw !== null && 'default' in raw) {
        return (raw as any).default as Component;
    }

    // Case 3: It's a plain component definition
    return raw as Component;
}

const renderedComponents = computed(() => {
    const results: Component[] = [];
    const pluginIds = pluginManager.plugins().value; // reactive array of plugin IDs

    for (const pid of pluginIds) {
        const plugin = pluginManager.getPlugin(pid);
        if (!plugin) continue;

        try {
            const raw = plugin.renderAnchor(props.id);
            if (raw === undefined || raw === null) continue;

            const comp = normalizeComponent(raw);
            if (comp) {
                results.push(comp);
            }
        } catch (e) {
            console.error(`Plugin ${pid} failed to render anchor ${props.id}:`, e);
        }
    }

    return results;
});
</script>

<style lang="scss" scoped>
.torii-anchor {
    display: flex;
    flex-direction: column;
    justify-content: start;
    width: 100%;
    height: 100%;
}
</style>
