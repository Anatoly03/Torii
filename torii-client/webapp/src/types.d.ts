/**
 * @file  index.d.ts
 * @brief This file contains the TypeScript bindings for Rust items used by the
 *        Torii application.
 */

/** The record struct represents a Torii record. */
export interface Record {
    /**
     * The [Workspace] that the record belongs to.
     * @example
     *
     * {
     *     "path": "/home/user/projects/workspace",
     *     "name": "workspace"
     * }
     */
    workspace: Workspace;

    /**
     * The full file system path to the record, relative to the [Workspace] directory.
     * @example
     *
     * "/home/user/projects/workspace/path/to/record"
     */
    path: string;

    /**
     * The full file system path to the record, relative to the [Workspace] directory.
     * @example
     *
     * "path/to/record"
     */
    relative_path: string;

    /**
     * The name of the record, which is the last component of the relative path.
     * @example
     *
     * "record"
     */
    name: string;
}

/** The workspace struct represents a Torii workspace. */
export interface Workspace {
    /**
     * The file system path to the workspace.
     * @example
     *
     * "/home/user/projects/workspace"
     */
    path: string;

    /**
     * The name of the workspace, which is the last component of the workspace path.
     * @example
     *
     * "workspace"
     */
    name: string;
}

/**
 * The component meta represents component metadata such as command capabilities.
 */
export interface ComponentMeta {
    /**
     * The name of the component.
     */
    name: string;

    /**
     * The permissions matrix.
     */
    permissions: {
        /** Wether the Tauri command `save_record_component` is implemented for the component. */
        write: bool;

        /** Wether the Tauri command `save_record_component_from_local_file` is implemented for the component. */
        write_from_file: bool;

        /** Wether the Tauri command `get_record_component` is implemented for the component. */
        read: bool;

        /** Wether the Tauri command `remove_record_component` is implemented for the component. */
        remove: bool;
    };
}
