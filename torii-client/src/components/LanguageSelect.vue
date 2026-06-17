<template>
    <select v-model="$i18n.locale" class="locale-select">
        <option value="en">English</option>
        <option value="jp">日本語</option>
        <option value="zh">中文</option>
    </select>
</template>

<style lang="scss" scoped>
.locale-select {
    flex: 1;
}
</style>
