/**
 * @file plugin-loader.ts
 */

import ToriiPlugin from './plugin';
import ToriiPluginEvents from './plugin-events';

/**
 * @brief A Torii plugin loader is a scope of code.
 */
export class ToriiPluginLoader {
    /**
     * @brief Currently installed plugins.
     */
    private readonly plugins: ToriiPlugin[] = [];

    /**
     * @todo
     */
    constructor() {}

    /**
     * @brief Install a plugin.
     */
    public install(plugin: ToriiPlugin) {
        this.plugins.push(plugin);
        plugin.emit('plugin-activate');
    }

    /**
     * @brief Emits an event.
     */
    public emit<Event extends keyof ToriiPluginEvents>(
        e: Event,
        ...args: Parameters<ToriiPluginEvents[Event]>
    ): this {
        this.plugins.forEach((plugin) => plugin.emit(e, ...args));
        return this;
    }
}

export default ToriiPluginLoader;
