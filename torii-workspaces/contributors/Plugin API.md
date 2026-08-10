# Plugin Architecture Discussion

The following is a code idea how a Torii plugin could look like. The vision is, similar to Obsidian, to provide a powerful interface for Torii plugins and modularize the codebase into extensions.

The plugin API should expose:

- ProseMirror API
- TipTap API
- Tauri API

### Example

```
import Torii from "@torii/api"

/** 
 * Example Torii Component
 */
export CustomComponent = Torii.Component({
  name: "custom-component",

  /**
   * In case component wants to store some custom data in
   * entity.config.json, it can be defined here.
   */
  provideConfig: () => ({
    Torii.BooleanSetting("hide-something"),
    Torii.RangeSetting("offset-y", { min: -1, max: 1 }),
  }),

  /**
   * If this method is defined, then components' source code
   * can be manually edited.
   */
  isSourceEeditable: true

  /**
   * If this method is defined, then component will be
   * rendered for view mode.
   */
  renderViewMode() {
    return await import("./ComponentView.vue");
  }

  /**
   * If this method is defined, then component will be
   * rendered for edit mode.
   */
  renderEditMode() {
    return await import("./ComponentEdit.vue");
  }
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
  provideLocales: () => ({
    "en": {
      "custom-keybind:label": "Custom Keybind",
      "custom-bool:enable": "Custom Boolean Enabled",
      "custom-bool:disable": "Custom Boolean Disabled",
      "custom-range:label": "Custom Range",
    }
  })
});
```

### Notes

It's still planned.



&nbsp;