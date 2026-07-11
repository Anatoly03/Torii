### `v0.0.0-1` - GitHub CI Test

-   [x] Verify the continuous integration works and can build artefacts.

# Hajime 始め

### `v0.0.0-dev2` [COMPLETED]

The first stage of the project is to create a simple file browser like application with tools to create and manage markdown notes.

-   [x] Setup Client Files
-   [x] Setup Desktop Files
-   [x] Launcher View
    -   [x] View Recent Projects
    -   [x] Delete Recent Project
    -   [x] Open Recent Project
    -   [x] Create Project (at directory)
-   [x] Settings
    -   [x] General: Language Select
    -   [x] Themes Stub
-   [x] Project View
    -   [x] View File Tree (non-recursive, only root markdown files)
    -   [x] Markdown: Edit and Auto-Save Files
    -   [x] Markdown: Create Files
    -   [x] Markdown: Delete Files
    -   [x] Close Project
-   [x] Simple Localization: EN, JP, ZH

### `v0.0.0-dev3` [COMPLETED]

-   [x] Create demo project which is linked in the repository (only accessible in dev builds)
    -   [x] Define the project Vision
-   [x] Rewrite Tauri code to support component system stubs.
-   [x] Image Component: Ability to view images.

### `v0.0.0-dev4` [COMPLETED]

-   [x] Fix CSS on Apple devices
-   [x] Add tabulation support to navigate the project
-   [x] Fix file tree state and disable ability to deselect file.
-   [x] Extend Markdown
    -   [x] Autocomplete File Links
    -   [x] File Links Support (Click)

### `v0.0.0-dev5` [COMPLETED]

-   [x] Add Image component functionality: Create/ Replace Image
    -   [x] Drag and Drop
    -   [x] Click to open file dialog and select
-   [x] Fix Drag and Drop error with local file paths using Japanese locale.
-   [x] Add Image Banner component
-   [x] Scrolling should remove autocomplete popup.
-   [x] Fix program crash when closing project to go to launcher (Markdowns' TipTap Editor is destroyed but still invoked)

### `v0.0.0-dev6` [COMPLETED]

-   [x] Add Windows builds to publish workflow.
    -   [x] Add Windows versioning fixer script
-   [x] Fix hot reload bug where settings can't be opened after reload.
-   [x] Add footer
    -   [x] Word Count
-   [x] Add Settings System
    -   [x] Enable/ Disable Word Count
    -   [x] Synchronize setting changes between Tauri windows
    -   [x] Store settings persisted on the file system

### `v0.0.0-dev7`[CURRENT]

-   [x] Add directory component: If entity path is a directory, it can contain records
-   [x] Add recursion support to file system UI, ability to display child records
-   [x] Drag and drop support to move records into directories
-   [ ] Cache all records on start for autocomplete (or scan recursive)
-   [ ] Re-add ability to name new or rename existing files.
-   [ ] Test Drag & Drop inbetween nested folders
-   [ ] Add new workspace which gets saved to resources: Changelog, Quick Guide

### `v0.0.0-dev8`

-   [ ] Rust Refactoring
    -   [ ] Component methods should be e.g. `remove() -> Option<Fn -> Result>`: None if no action is defined, and it should return a lazy callback.
-   [ ] Breadcrumb Header
-   [ ] Client Refactoring (Types, Documentation Github CI)
    -   [ ] Remove Naive UI dependency and implement custom vue components

### `v0.0.0-dev9`

-   [ ] Expand Footer
    -   [ ] Edit/ View mode (hides placeholder grids vs. allows to edit article & enables drag and drop)
-   [ ] Fix bug where opening new record sometimes scrolls to an offset.
-   [ ] Opt-In Setting: Citation Tracking Component (Store URLs for uploaded Images and inserted content.)

### `v0.0.0-dev10`

-   [ ] Define CSS variables, modularize CSS
-   [ ] Refactor to Theme Support
    -   [ ] Light Mode
    -   [ ] Dark Mode
-   [ ] Add Record search (filter records by search pattern)

### `v0.0.0-dev11`

-   [ ] Extend Markdown
    -   [ ] Image embed support (new images are stored as a child-record in this records' directory)
-   [ ] Add index tracking for backlinks (records referencing other records, maintain pointers)
-   [ ] Add ability to rename files with fixes in backlinks
    -   [ ] Add component trait method `rename_link`: Invoked on records with backlinks, rename link within component.

### `v0.0.0-dev12`

-   [ ] Export one record or entire workspace
    -   [ ] PDF
    -   [ ] HTML
    -   [ ] LaTeX

### `v0.0.0-dev13`

-   [ ] Refactor Code
    -   [ ] Create a client-side plugin abstraction in `client-core`, but no dynamic plugin loader yet.
-   [ ] Github CI
    -   [ ] cargo test
    -   [ ] cargo doc
    -   [ ] typescript docs

### `v0.0.0-dev14`

-   [ ] Refactor to an embeddable localization system.
    -   [ ] In dev builds, they are linked with the repository.
    -   [ ] In production, they are linked with the [resources](https://v2.tauri.app/develop/resources/).
-   [ ] Add a settings tab to allow creating new localizations.
    -   [ ] All localization key-values should be editable.
    -   [ ] User Interface for vertical line: Singular, plural, with bracket placeholders.

### `v0.0.0-dev15`

-   [ ] Refactor to an embeddable client-plugin system. (Currently, statically link all plugins, no dynamic loading)
-   [ ] General Keybinds and keybinds to overwrite [TipTap Keybinds](https://tiptap.dev/docs/editor/core-concepts/keyboard-shortcuts)

### `v0.0.0-dev16`

-   [ ] Add project icon
-   [ ] FULLY COVERED CODE REVIEW BEFORE FIRST RELEASE
-   [ ] Full Code Documentation

# Daiishi 台石

### `v0.0.1`

-   [ ] Add support for [automatic updates](https://v2.tauri.app/plugin/updater/)

### `v0.0.1-dev1`

-   [ ] Refactor to an embeddable theme system.
    -   [ ] In dev builds, they are linked with the repository styles.
    -   [ ] In production, they are linked with the [resources](https://v2.tauri.app/develop/resources/).

### `v0.0.1-dev2`

-   [x] Torii API
    -   [ ] Blogposts (Main Menu)
    -   [ ] Torii Auth (reserved, not used, no registration)
-   [ ] Dynamically load client-plugins
-   [ ] Publish client-extension API [to package managers](https://github.com/Anatoly03/Torii/packages)

# Nemaki 根巻

### `v0.0.2`

-   [ ] TBA: Backend extensions

<!-- Daiishi / Kamebara -->
<!-- Hashira -->
<!-- Nuki -->
<!-- Kasagi -->
<!-- Hafu -->
