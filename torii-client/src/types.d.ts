/**
 * @file  index.d.ts
 * @brief This file contains the TypeScript bindings for Rust items used by the
 *        Torii application.
 */

/** The record struct represents a Torii record. */
export interface Record {
	/** The [Workspace] that the record belongs to. */
	public workspace: Workspace;

	/**
     * The full file system path to the record, relative to the [Workspace] directory.
     * @example
     * 
     * "file:///home/user/projects/workspace/path/to/record"
     */
	public path: string;

	/** The name of the record, which is the last component of the relative path. */
	public name: string;
}

/** The workspace struct represents a Torii workspace. */
export interface Workspace {
	/** The file system path to the workspace. */
	public path: string;

	/** The name of the workspace, which is the last component of the workspace path. */
	public name: string;
}
