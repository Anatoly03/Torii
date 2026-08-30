/**
 * @file plugin-events.ts
 * @brief Defines all internally-understood Torii plugin events.
 */

/**
 * List of internallly-understood Torii events. A plugin can always emit custom
 * events, but these will be ignored by internal Torii plugins.
 */
export type ToriiPluginEvents = {
    // plugin lifecycle
    ['plugin-activate']: () => void;
    ['plugin-deactivate']: () => void;

    // workspace lifecycle
    ['workspace-open']: (path: string) => void,
    ['workspace-close']: (path: string) => void,

    // record lifecycle
    ['record-open']: () => void;
    ['record-close']: () => void;
    ['record-create']: () => void;
    ['record-rename']: () => void;
    ['record-delete']: () => void;

    // records' individual component lifecycle
    ['record-component-create']: () => void;
    ['record-component-update']: () => void;
    ['record-component-delete']: () => void;
};

export default ToriiPluginEvents;
