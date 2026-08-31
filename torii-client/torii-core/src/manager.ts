import { readonly, ref } from 'vue';
import ToriiPlugin from './plugin.ts';

/**
 * @brief The Torii plugin manager.
 */
export default class ToriiPluginManager {
    #plugins: ToriiPlugin[] = [];
    #pluginNames = ref<string[]>([]);

    /**
     * @brief Gets the reactive reference of installed plugins. This is immutable outside
     * of the Torii plugin manager.
     */
    public plugins() {
        return readonly(this.#pluginNames);
    }

    /**
     * @brief Installs a new plugin to the manager.
     */
    public install(p: ToriiPlugin) {
        this.#plugins.push(p);
        this.#pluginNames.value.push(p.id);
    }

    /**
     * @brief Uninstalls a plugin from the manager.
     */
    public uninstall(pluginId: string) {
        this.#plugins = this.#plugins.filter((p) => p.id !== pluginId);
        this.#pluginNames.value = this.#pluginNames.value.filter(
            (pid) => pid !== pluginId
        );
    }

    /**
     * @brief Retrieves a plugin from the manager by its identifier.
     */
    public getPlugin(pluginId: string): ToriiPlugin | undefined {
        return this.#plugins.find((p) => p.id === pluginId);
    }
}
