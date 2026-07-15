# Plugin Architecture Discussion

The following is a code idea how a Torii plugin could look like. The vision is, similar to Obsidian, to provide a powerful interface for Torii plugins and modularize the codebase into extensions.

### Example

```
import Torii from "@torii/api"

/** 
 * Example Torii Component
 */
export CustomComponent = Torii.Component({
  // TODO
})

/**
 * Example Torii Plugin 
 */
export default Torii.Plugin({
  name: "example-plugin",

  /**
   * Plugins can provide custom keybinds.
   */
  provideKeybinds: () => [
    Torii.Keybind("custom-keybind", {
      default: ["Cmd+Click"]
    })
  ],

  /**
   * Plugins can provide custom settings.
   */
  provideSettings: () => [
    Torii.BooleanSetting("custom-bool"),
    Torii.RangeSetting("custom-range", {
      min: 0, max: 10, step: 1, default: 3,
    }),
  ],

  /**
   * Provide custom components
   */
  provideComponents: () => [
    CustomComponent
  ],

  /**
   * Plugins can provide custom locale extensions. Plugins
   * have to manage their own provided text values themself,
   * but "locale resources" can provide localizations for
   * plugins too.
   * They will be mapped to <locale>.<plugin name>.<...>
   * e.g. en.example-plugin.custom-keybind:label
   */
  provideLocales: () => {
    "en": {
      "custom-keybind:label": "Custom Keybind",
      "custom-bool:enable": "Custom Boolean Enabled",
      "custom-bool:disable": "Custom Boolean Disabled",
      "custom-range:label": "Custom Range",
    }
  }
});
```

### Notes

It's still planned.



&nbsp;