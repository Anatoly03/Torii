### `v0.0.0-1` - GitHub CI Test

-   [x] Verify the continuous integration works and can build artefacts.

### `v0.0.0-dev2` Hajime 始め [COMPLETED]

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

### `v0.0.0-dev7` [COMPLETED]

-   [x] Add directory component: If entity path is a directory, it can contain records
-   [x] Add recursion support to file system UI, ability to display child records
-   [x] Drag and drop support to move records into directories
-   [x] Add Refresh button
-   [x] Re-add ability to name new or rename existing files.
-   [x] Re-add: Open "README" on project opening.
-   [x] Fix Drag & Drop inbetween nested folders
-   [x] Fix command `list_records` providing wrong workspace path.
-   [x] Add new workspace which gets saved to resources: Changelog, Quick Guide

### `v0.0.0-dev8` [COMPLETED]

-   [x] Add `.vscode` launch configuration
-   [x] Implement click for remote URLs
-   [x] Fix: Editing renamed file saves to outdated file path.
-   [x] Scan all records recursive for autocomplete
-   [x] Github CI: `cargo test`
-   [x] Rust Refactoring
    -   [x] Component methods should be e.g. `remove() -> Option<Fn -> Result>`: None if no action is defined, and it should return a lazy callback.
    -   [x] Refactor `list_record_components` event to yield component permissions
-   [x] Add Record search (filter records by search pattern)
-   [x] Set up branch protection rules (block force pushes)

### `v0.0.0-dev9` [COMPLETED]

-   [x] Client Documentation (Typedoc Infrastructure)
-   [x] Expand Footer with Edit and View mode.
    -   [x] View Mode: Fix layout/ spacing when image or banner is gone.
-   [x] Refactor workspaces to scale more easily when adding default workspaces. (`workspaces.json`)

Due to my Bachelors' thesis I took a 3 week break from Torii here and `dev9` should be a small update to bring back the pace.

-   [x] Add support for [automatic updates](https://v2.tauri.app/plugin/updater/) by adding artifacts (build signatures) to CI/CD.
-   [x] Add a button which checks if an update is available. (Not Tested!)

### `v0.0.0-dev10` [NEXT]

-   [x] Client Refactoring (Vue webapp and plugins first steps)
-   [ ] Edit Mode: WYSIWYG if provided, allows to edit article, drag and drop
-   [ ] Edit Mode: Remove: Open URL on link click
-   [ ] View Mode: Hides placeholder grids and no interaction
-   [ ] View Mode: Hide autocomplete
-   [ ] View Mode: Fix: Link should be clickable
    -   [ ] Remote URL
    -   [ ] Local Files/ Relative Paths
-   [ ] Breadcrumb Header
-   [ ] Remove: Control + click.
-   [ ] Implement Rust path normalization in workspace and record
-   [ ] Autocomplete should fill encoded paths (e.g. space should become `%20`)

### `v0.0.0-dev11`

-   [ ] Client Documentation (Github CI)
    -   [ ] [clean-jsdoc-theme](https://www.npmjs.com/package/@clean-jsdoc-theme/typedoc)
-   [ ] Remove Naive UI dependency and implement custom vue components
-   [ ] Expand Footer with Source mode.
    -   [ ] Source Mode: Raw source of the file, only when MIME type is not a binary.
    -   [ ] Source Mode: Use [codemirror](https://codemirror.net)

### `v0.0.0-dev12`

-   [ ] Add `entity.config.json` API
-   [ ] Add component manager collapsable sidebar
    -   [ ] View Attached Components (+ meta)
    -   [ ] Add Component from Selection
    -   [ ] Remove Component
    -   [ ] Edit Component Config
-   [ ] Add Banner component config: Offset Y, Offset X, Zoom
-   [ ] Render Banner image with offset

### `v0.0.0-dev13`

-   [ ] Define CSS variables, modularize CSS
-   [ ] Add UX to move banner offset in edit mode. (scroll, zoom)
-   [ ] Add infoboxes
    -   [ ] Infobox Definition Component: Table, Attributes, Types
    -   [ ] Values stored `config.json`, as well as relative component definition path.

### `v0.0.0-dev14`

-   [ ] Refactor to Theme Support
    -   [ ] Light Mode
    -   [ ] Dark Mode
-   [ ] Opt-In Setting: Citation Tracking Component (Store URLs for uploaded Images and inserted content.)
-   [ ] Fix bug where opening new record sometimes scrolls to an offset.

### `v0.0.0-dev15`

-   [ ] Extend Markdown
    -   [ ] Image embed support (new images are stored as a child-record in this records' directory)
-   [ ] Add index tracking for backlinks (records referencing other records, maintain pointers)
-   [ ] Add ability to rename files with fixes in backlinks
    -   [ ] Add component trait method `rename_link`: Invoked on records with backlinks, rename link within component.

### `v0.0.0-dev16`

-   [ ] Export one record
    -   [ ] PDF
    -   [ ] HTML
    -   [ ] LaTeX

### `v0.0.0-dev17`

-   [ ] Export one record
    -   [ ] LaTeX
-   [ ] Export entire workspace
    -   [ ] PDF
    -   [ ] HTML
-   [ ] Refactor Code
    -   [ ] Create a client-side plugin abstraction in `client-core`, but no dynamic plugin loader yet.
-   [ ] Github CI
    -   [x] cargo test
    -   [ ] cargo doc
    -   [ ] typescript docs
    -   [ ] Github [Labeler](https://github.com/actions/labeler)
    -   [ ] Security: `cargo deny`
    -   [ ] Security: `cargo audit`, `npm audit`

### `v0.0.0-dev18`

-   [ ] Refactor to an embeddable localization system.
    -   [ ] In dev builds, they are linked with the repository.
    -   [ ] In production, they are linked with the [resources](https://v2.tauri.app/develop/resources/).
-   [ ] Add a settings tab to allow creating new localizations.
    -   [ ] All localization key-values should be editable.
    -   [ ] User Interface for vertical line: Singular, plural, with bracket placeholders.

### `v0.0.0-dev19`

-   [ ] Refactor to an embeddable client-plugin system. (Currently, statically link all plugins, no dynamic loading)
-   [ ] General Keybinds and keybinds to overwrite [TipTap Keybinds](https://tiptap.dev/docs/editor/core-concepts/keyboard-shortcuts)

### `v0.0.0-dev20`

-   [ ] Add project icon
-   [ ] FULLY COVERED CODE REVIEW BEFORE FIRST RELEASE
-   [ ] Full Code Documentation
-   [ ] Enable "Require PR before merging" to branch protection rules (require approvals)

### `v0.0.1` Daiishi 台石

-   [ ] Refactor to an embeddable theme system.
    -   [ ] In dev builds, they are linked with the repository styles.
    -   [ ] In production, they are linked with the [resources](https://v2.tauri.app/develop/resources/).

### `v0.0.1-dev1`

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
