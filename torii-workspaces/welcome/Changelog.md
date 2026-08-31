# Changelog

---

### [v0.0.0-dev10](https://github.com/Anatoly03/Torii/releases/tag/app-v0.0.0-dev10)

- Add plugin: Recent Projects
- Add a settings page to display installed plugins.
- Fix: Hyperlinks no longer open in Edit Mode.
- Fix: Autocomplete no longer pops up in Preview Mode.

Technical Change: The codebase was refactored to support plugins which are dynamically loaded. This is still premature for usage by 3rd party extensions, but will act as a solid base in the future.

---

### [v0.0.0-dev9](https://github.com/Anatoly03/Torii/releases/tag/app-v0.0.0-dev9)

- Add "Preview Mode" to record footer, which disable editing and hides image placeholder grids.
- Add an update information button in the welcome screen. *This button has not been tested yet! We will see when dev10 releases if it works.*

Technical Change: The Github CI/CD now builds releases with signatures. This release comes with an update button that is capable of update the app to a newer version.

---

### [v0.0.0-dev8](https://github.com/Anatoly03/Torii/releases/tag/app-v0.0.0-dev8)

- Add search bar in the file tree.
- Add feature to open browser links when clicked.
- Cache autocomplete suggestions.

For Developers: Added new workspace "Torii Contributors" for maintaining programmer resources in-app.

---

### [v0.0.0-dev7](https://github.com/Anatoly03/Torii/releases/tag/app-v0.0.0-dev7)

- Add support to for the "folder" component.
- Add ability to rename records by double clicking their file entry.
- Add project sidebar quick menu: New File, Refresh Files. Refresh Files will preserve opened folder structure.
- Add a "welcome workspace" to the production build. (If you read this you're in the welcome workspace right now!)
- Technical: Rewire the autocomplete feature to see cached files. (You need to open the folder in the file tree for it to be visible by autocompletion)
- Technical: A lot of client-side refactoring, rewrite the project sidebar with custom tree implementation.

---

### [v0.0.0-dev6](https://github.com/Anatoly03/Torii/releases/tag/app-v0.0.0-dev6)

- Add a setting which can enable a project footer containg the article word count.
- Add some missing localizations.

---

### [v0.0.0-dev5](https://github.com/Anatoly03/Torii/releases/tag/app-v0.0.0-dev5)

- Add drag and drop feature to create the image component. You can now also click the placeholer to open a file dialog.
- Add decorative banner component which is behind the header image.
- Scrolling in a project now removes the autocomplete popup.
- Settings can now be accessed from project view.
- Fix minor bug with trying to drag and drop images whose path included characters in a non-English locale.

---

### [v0.0.0-dev4](https://github.com/Anatoly03/Torii/releases/tag/app-v0.0.0-dev4)

- Add minor support for tabulation accross the project.
- Added very simple autocomplete feature for linking markdown notes (e.g. [Vision](./Vision.md), [Article](./Article.md)).
- Sort all files in the file tree alphabetically.
- Fix minor CSS issuesses on Mac.
- Fix minor bug with recent projects where "Torii Dev" workspace would duplicate itself into the local project list.
- Fix state reactivity with sidebar file tree and prevent unselecting files.

---

### [v0.0.0-dev3](https://github.com/Anatoly03/Torii/releases/tag/app-v0.0.0-dev3)

- Added technical functionality to display one header image.

---

### [v0.0.0-dev2](https://github.com/Anatoly03/Torii/releases/tag/app-v0.0.0-dev2) **Hajime 始め**

- View, (Add) Open &amp; Unlink Recently Opened Projects
- View, Create, Edit &amp; Remove Markdown Files (`.md`)
- Added Localization Support (English, Japanese, Chinese)

---

