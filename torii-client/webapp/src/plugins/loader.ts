import { ToriiPluginManager } from '@torii-project/core';
import { RecentProjectsPlugin } from '@torii-project/plugin-recent-projects';

/**
 * @brief Singleton instance of the Torii plugin manager.
 */
let singleton: ToriiPluginManager | undefined;

/**
 * @brief When the plugins are initialized, it installs the core plugins.
 */
export function installCorePlugins(m: ToriiPluginManager) {
    m.install(new RecentProjectsPlugin());
}

/**
 * @returns The singleton instance of the Torii plugin manager, lazily
 * initializing it if is not yet set.
 */
export function usePlugins() {
    if (!singleton) {
        singleton = new ToriiPluginManager();
        installCorePlugins(singleton);
    }

    return singleton;
}
