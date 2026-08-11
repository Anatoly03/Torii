/**
 * @file plugin.ts
 * @brief Defines the Torii Plugin extensible.
 */

export interface ToriiPluginSettings {
    onInit?: () => void;
    onDestroy?: () => void;
}

/**
 * @brief A Torii plugin is a scope of code, with its' own lifecycle, that
 * can be loaded, interacted with and offloaded by Toriis' internal plugin
 * loader.
 */
export class ToriiPlugin {
    //
    // ATTRIBUTES, INTERNAL PLUGIN SETTINGS
    //

    //
    // CONSTRUCTOR
    //
    constructor() {}

    //
    // STATIC FUNCTIONS
    //

    /**
     * @brief Quickly create a new, small Torii plugin from the provided 
     * settings object. For more complex Torii types, consider extending
     * the class.
     * 
     * @example
     * 
     * ToriiPlugin.create({
     *     // TODO
     * });
     * 
     * @returns Newly created Torii Plugin.
     */
    public static create(_settings: ToriiPluginSettings): ToriiPlugin {
        const plugin = new ToriiPlugin();

        // TODO

        return plugin;
    }

    //
    // METHODS
    //
}
