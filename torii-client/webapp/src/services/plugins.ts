import { ToriiPluginLoader } from '@torii/plugin-core';
import ToriiDebugPlugin from '@torii/plugin-debug'

let pluginLoader: ToriiPluginLoader | undefined;

export function usePlugins(): ToriiPluginLoader {
    if (pluginLoader) return pluginLoader;
    
    pluginLoader = new ToriiPluginLoader();
    pluginLoader.install(ToriiDebugPlugin);

    return pluginLoader;
}
