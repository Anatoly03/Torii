<template>
    <div class="view-settings">
        <div class="view-settings-side">
            <router-link
                v-for="subsetting in subsettings"
                :key="subsetting.path"
                :to="subsetting.path"
            >
                {{ subsetting.name }}
            </router-link>
        </div>
        <div class="view-settings-content">
            <router-view />
        </div>
    </div>
</template>

<script setup lang="ts">
const subsettings = [
    {
        name: 'General',
        path: '/settings/general',
    },
    {
        name: 'Theme',
        path: '/settings/theme',
    },
];
</script>

<style lang="scss" scoped>
.view-settings {
    display: flex;
    flex-direction: row;
    height: 100%;

    .view-settings-side {
        display: flex;
        flex-direction: column;
        width: 200px;
        padding: 16px;
        gap: 8px;
        border-right: 1px solid #ccc;
    }

    .view-settings-content {
        display: flex;
        flex: 1;
        flex-direction: column;
        align-items: center;
        padding: 16px;
    }
}
</style>
