import { ToriiPluginManager } from '@torii-project/core';

/**
 * @brief Singleton instance of the Torii plugin manager.
 */
let singleton: ToriiPluginManager | undefined;

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
