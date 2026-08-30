import { ToriiPlugin, ToriiPluginEvents } from '@torii/plugin-core';

class ToriiDebugPlugin extends ToriiPlugin {
    public override pluginVersion = '0.1.0';

    constructor() {
        super('core:debug:plugin-debug');
    }

    /**
     * @brief Print the received event information to the console.
     */
    public override emit<Event extends keyof ToriiPluginEvents>(
        e: Event,
        ...args: Parameters<ToriiPluginEvents[Event]>
    ): this {
        // super.emit(e, ...args);
        console.debug(`[plugin-debug] Event '${e}':`, ...args);
        return this;
    }
}

export default new ToriiDebugPlugin();
