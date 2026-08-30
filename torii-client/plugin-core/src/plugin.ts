/**
 * @file plugin.ts
 * @brief Defines the Torii Plugin extensible.
 */

import ToriiPluginEvents from './plugin-events';

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
     *   id: "core:markdown:hyperlinks",
     * }
     */
    id: string;

    /**
     * Semver string of the plugin version, consisting of at least major,
     * minor and optionally patch.
     * 
     *
     * @example
     *
     * {
     *   version: "1.2.0-dev4",
     * }
     */
    version: string;

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
     *   id: "core:markdown:hyperlinks",
     * }
     */
    public readonly pluginId: string;

    /**
     * Semver string of the plugin version, consisting of at least major,
     * minor and optionally patch.
     *
     * @example
     *
     * {
     *   version: "1.2.0-dev4",
     * }
     */
    public pluginVersion: string = '0.1.0';

    //
    // Handles
    //

    /**
     * List of registered event handlers. The key is any understood Torii
     * event name, and the handler is an array of functions which listen
     * to the event.
     */
    #handlers: Map<
        keyof ToriiPluginEvents,
        ToriiPluginEvents[keyof ToriiPluginEvents][]
    > = new Map();

    //
    // ATTRIBUTES [CALLBACKS]
    //

    //
    // CONSTRUCTOR
    //
    constructor(pluginId: string) {
        this.pluginId = pluginId;
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
        const plugin = new ToriiPlugin(settings.id);

        if (settings.onActivate)
            plugin.on('plugin-activate', settings.onActivate);
        if (settings.onDeactivate)
            plugin.on('plugin-deactivate', settings.onDeactivate);

        return plugin;
    }

    //
    // METHODS
    //

    // TODO

    //
    // EVENTS
    //

    /**
     * @brief Register a callback to be invoked when the plugin is activated.
     * This can happen multiple times over the runtime of the plugin.
     *
     * For destructor behaviour when the plugin is deactivated, you should
     * use {@link ToriiPluginSettings.onDeactivate|onDeactivate}.
     */
    on(e: 'plugin-activate', callback: () => void): this;

    /**
     * @brief Register a callback to be invoked when the plugin is deactivated,
     * either by application closing or manual plugin deactivation by the user.
     *
     * For constructor behaviour when the plugin is activated, you should
     * use {@link ToriiPluginSettings.onActivate|onActivate}.
     */
    on(e: 'plugin-deactivate', callback: () => void): this;

    // TODO workspace-open
    // TODO workspace-close

    /**
     * @brief Register a callback to be invoked when a record was opened.
     *
     * For behaviour when the record is closed, you should use
     * {@link ToriiPluginSettings.onRecordClose|onRecordClose}.
     */
    on(e: 'record-open', callback: () => void): this;

    /**
     * @brief Register a callback to be invoked when a record was closed. When
     * a new record is opened instead, the old record will be closed first.
     *
     * For behaviour when the record is opened, you should use
     * {@link ToriiPluginSettings.onRecordOpen|onRecordOpen}.
     */
    on(e: 'record-close', callback: () => void): this;

    /**
     * @brief Register a callback to be invoked when a record was created.
     */
    on(e: 'record-create', callback: () => void): this;

    /**
     * @brief Register a callback to be invoked when a record was renamed.
     */
    on(e: 'record-rename', callback: () => void): this;

    /**
     * @brief Register a callback to be invoked when a record was deleted.
     */
    on(e: 'record-delete', callback: () => void): this;

    /**
     * @brief Register a callback to be invoked when a record was attached
     * with a new component.
     */
    on(e: 'component-create', callback: () => void): this;

    /**
     * @brief Register a callback to be invoked when a records' component was
     * modified.
     */
    on(e: 'component-update', callback: () => void): this;

    /**
     * @brief Register a callback to be invoked when a records' component was
     * deleted.
     */
    on(e: 'component-delete', callback: () => void): this;

    /**
     * @brief Registers a new event handler.
     * @param e Name of the event to listen for. When the plugin loader receives
     * this event, the callback function will be invoked.
     */
    public on<Event extends keyof ToriiPluginEvents>(
        e: Event,
        callback: ToriiPluginEvents[Event]
    ): this {
        switch (this.#handlers.has(e)) {
            case true:
                this.#handlers.get(e)!.push(callback);
                return this;
            case false:
                this.#handlers.set(e, [callback]);
                return this;
        }
    }

    /**
     * @brief Registers a new event handler.
     * @param e Name of the event to listen for. When the plugin loader receives
     * this event, the callback function will be invoked.
     */
    public emit<Event extends keyof ToriiPluginEvents>(
        e: Event,
        ...args: Parameters<ToriiPluginEvents[Event]>
    ): this {
        this.#handlers.get(e)?.forEach((callback: (...e: any) => void) => {
            callback(...args);
        });
        return this;
    }
}

export default ToriiPlugin;
