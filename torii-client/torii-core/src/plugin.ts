import type { Component } from 'vue';

/**
 * @brief Torii plugin initialization settings
 */
export type ToriiPluginConfig = {
    id: string;
    version: string;
};

/**
 * @brief A Torii plugin
 */
export default class ToriiPlugin {
    public readonly id: string;
    public readonly version: string;

    constructor(config: ToriiPluginConfig) {
        this.id = config.id;
        this.version = config.version;
    }

    /**
     * @brief This function is invoked by the internal `<TAnchor>` element and renders
     * content at specific anchor points. Plugins can append custom content to these
     * predefined anchor points.
     *
     * @param anchorId The name of the anchor id.
     */
    public renderAnchor(anchorId: string): Promise<Component> | Component | undefined {
        void anchorId; // mark anchorId as used
        return undefined;
    }
}
