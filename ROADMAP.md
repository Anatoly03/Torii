### `v0.0.0-1` - GitHub CI Test

-   [x] Verify the integration works and can build artefacts.

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

### `v0.0.0-dev6` [CURRENT]

-   [x] Add Windows builds to publish workflow.
    -   [x] Add Windows versioning fixer script
-   [ ] Add footer to editing components
    -   [ ] Word Count
    -   [ ] Edit/ View mode (hides placeholder grids vs. allows to edit article & enables drag and drop)
-   [ ] Add Settings
    -   [ ] Enable/ Disable Word Count
    -   [ ] Keybinds
-   [ ] Extend Markdown
    -   [ ] Image embed support

### `v0.0.0-dev7`

-   [ ] Fix bug where opening new record sometimes scrolls to an offset.
-   [ ] Refactor to Theme Support
    -   [ ] Light Mode
    -   [ ] Dark Mode

### `v0.0.0-dev8`

-   [ ] Add index tracking for backlinks (records referencing other records, maintain pointers)
-   [ ] Add ability to rename files with fixes in backlinks
    -   [ ] Add component trait method: Invoked on records with backlinks, rename link within component.
-   [ ] Add recursion support to file system
-   [ ] Add workspace directories

# Daiishi 台石

### `v1.0.0`

-   [ ] Dynamic Theme Manager
    -   [ ] Write Default Themes into `$APPDATA/themes/**`
    -   [ ] View Themes
        -   [ ] SCSS support
        -   [ ] Theme name localization support.
-   [x] Torii API
    -   [ ] Blogposts (Main Menu)
    -   [ ] Torii Auth (reserved, not used, no registration)

# Nemaki 根巻

### `v1.0.1`

-   [ ] To-Do: Far future

<!-- Daiishi / Kamebara -->
<!-- Hashira -->
<!-- Nuki -->
<!-- Kasagi -->
<!-- Hafu -->
