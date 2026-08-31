import { readonly, ref } from 'vue';
import ToriiPlugin from './core';

/**
 * @brief Singleton instance of the Torii plugin manager.
 */
let singleton: ToriiPluginManager | undefined;

/**
 * @brief The Torii plugin manager.
 */
class ToriiPluginManager {
    #plugins: ToriiPlugin[] = [];
    #pluginNames = ref<string[]>([]);

    /**
     * @brief Gets the reactive reference of installed plugins. This is immutable outside
     * of the Torii plugin manager.
     */
    public plugins() {
        return readonly(this.#pluginNames);
    }

    public install(p: ToriiPlugin) {
        this.#plugins.push(p);
        this.#pluginNames.value.push(p.id);
    }

    public uninstall(pluginId: string) {
        this.#plugins = this.#plugins.filter((p) => p.id !== pluginId);
        this.#pluginNames.value = this.#pluginNames.value.filter(
            (pid) => pid !== pluginId
        );
    }

    public getPlugin(pluginId: string): ToriiPlugin | undefined {
        return this.#plugins.find((p) => p.id === pluginId);
    }
}

/**
 * @returns The singleton instance of the Torii plugin manager, lazily
 * initializing it if is not yet set.
 */
export function usePlugins() {
    if (!singleton) {
        singleton = new ToriiPluginManager();
    }

    return singleton;
}
