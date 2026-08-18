/**
 * @file plugin.ts
 * @brief Defines the Torii Plugin extensible.
 */

/**
 * Quick-creation settings for the static {@link ToriiPlugin.create()}
 * method.
 */
export interface ToriiPluginSettings {
    /**
     * @brief The unique identifier of the plugin, consisting of letters,
     * numerics and dashes. You can prefix the plugin with a custom scope
     * if needed. For example, `my-custom-scope:custom-plugin` would be an
     * allowed plugin name.
     *
     * @note The prefix `core:` should only be used by libraries internal
     * to the Torii project.
     *
     * @example
     *
     * {
     *   PLUGIN_ID: "core:markdown:hyperlinks",
     * }
     */
    PLUGIN_ID: string;

    // /**
    //  * @brief Callback to be invoked when the plugin is initialized. This
    //  * can happen only once if the plugin is initialized at start.
    //  *
    //  * @note You should **not** compute anything in this function that
    //  * "activates" the plugin. This function is explicitly only for logic
    //  * that should only run once in the lifetime of the plugin.
    //  *
    //  * For setup behaviour when a plugin (that was previously deactivated)
    //  * is enabled again see {@link ToriiPluginSettings.onActivate|onActivate}.
    //  */
    // onInit?: () => void;

    /**
     * @brief Callback to be invoked when the plugin is activated. This
     * can happen multiple times over the runtime of the plugin.
     *
     * For destructor behaviour when the plugin is deactivated, you should
     * use {@link ToriiPluginSettings.onDeactivate|onDeactivate}.
     */
    onActivate?: () => void;

    /**
     * @brief Callback to be invoked when the plugin is deactivated, either
     * by application closing or manual plugin deactivation by the user.
     *
     * For constructor behaviour when the plugin is activated, you should
     * use {@link ToriiPluginSettings.onActivate|onActivate}.
     */
    onDeactivate?: () => void;

    // onDestroy?: () => void;
}

/**
 * @brief A Torii plugin is a scope of code, with its' own lifecycle, that
 * can be loaded, interacted with and offloaded by Toriis' internal plugin
 * loader.
 */
export class ToriiPlugin {
    //
    // ATTRIBUTES [INTERNAL PLUGIN CONSTANTS]
    //

    /**
     * @brief The unique identifier of the plugin, consisting of letters,
     * numerics and dashes. You can prefix the plugin with a custom scope
     * if needed. For example, `my-custom-scope:custom-plugin` would be an
     * allowed plugin name.
     *
     * @note The prefix `core:` should only be used by libraries internal
     * to the Torii project.
     *
     * @example
     *
     * {
     *   PLUGIN_ID: "core:markdown:hyperlinks",
     * }
     */
    public PLUGIN_ID: string;

    //
    // Handles
    //

    #onActivateHandlers: (() => void)[] = [];
    #onDeactivateHandlers: (() => void)[] = [];

    //
    // ATTRIBUTES [CALLBACKS]
    //

    //
    // CONSTRUCTOR
    //
    constructor(pluginId: string) {
        this.PLUGIN_ID = pluginId;
    }

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
    public static create(settings: ToriiPluginSettings): ToriiPlugin {
        const plugin = new ToriiPlugin(settings.PLUGIN_ID);

        if (settings.onActivate) plugin.onActivate(settings.onActivate);
        if (settings.onDeactivate) plugin.onDeactivate(settings.onDeactivate);

        return plugin;
    }

    //
    // METHODS
    //

    /**
     * @brief Register a callback to be invoked when the plugin is
     * activated. This can happen multiple times over the runtime
     * of the plugin.
     *
     * For destructor behaviour when the plugin is deactivated, you should
     * use {@link ToriiPluginSettings.onDeactivate|onDeactivate}.
     */
    public onActivate(callback: () => void) {
        this.#onActivateHandlers.push(callback);
    }

    /**
     * @brief Register a callback to be invoked when the plugin is
     * deactivated, either by application closing or manual plugin
     * deactivation by the user.
     *
     * For constructor behaviour when the plugin is activated, you should
     * use {@link ToriiPluginSettings.onActivate|onActivate}.
     */
    public onDeactivate(callback: () => void) {
        this.#onDeactivateHandlers.push(callback);
    }
}
