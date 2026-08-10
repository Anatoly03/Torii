# Torii Workspaces

This folder contains all example workspaces, which will be visible when running the Torii application.

### Contributing (New Files)

If you want to contribute new records to existing workspaces, just run Torii in development mode and edit the files. Git will automatically track the file changes.

### Contributing (New Workspaces)

Usually there is no need to create new workspaces, but for the rare case when a new workspace is needed, please be aware to change the following:

- Create a new directory in this folder.
- In [workspaces.json](./workspaces.json), add a new entry for the workspace, giving it a name and the name of the newly created folder. If this repository should only be visible in dev builds, the `release` attribute should be `false`.
- If this workspace for some reason needs to end up in the release build, you need to link the newly created workspace in [tauri.conf.json](../torii-desktop/tauri.conf.json) as a resource. The repository is not copied to the workspace resources per default.
