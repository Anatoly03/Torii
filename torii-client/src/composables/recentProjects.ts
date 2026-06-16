/**
 * @file recentProjects.ts
 */

import { invoke } from '@tauri-apps/api/core';
import { Ref, ref } from 'vue';

/**
 *
 */
interface RecentProject {
    /**
     * The name of the project.
     */
    name: string;

    /**
     * The path to the project on disk.
     */
    path: string;

    /**
     * The timestamp of when the project was last opened, in milliseconds since
     * the Unix epoch.
     */
    lastOpened?: number;
}

const recentProjects = ref<RecentProject[]>([]);

/**
 * Communicates with the main process to get the list of recently opened
 * projects.
 * 
 * Returns a reactive reference to the list of projects, which will be
 * updated whenever the main process sends an update to the list of recent
 * projects.
 */
export function listRecentProjects(): Ref<RecentProject[]> {
    invoke<RecentProject[]>('list_recent_projects').then((projects) => {
        recentProjects.value = projects;
    });
    return recentProjects;
}

/**
 * Communicates with the main process to add a project to the list of recently
 * opened projects.
 */
export async function addRecentProject(metadata: RecentProject): Promise<void> {
    await invoke('add_recent_project', { metadata });
    listRecentProjects(); // update the list of recent projects ref
}

/**
 * Communicates with the main process to remove a project from the list of
 * recently opened projects.
 */
export async function removeRecentProject(path: string): Promise<void> {
    await invoke('remove_recent_project', { path });
    listRecentProjects(); // update the list of recent projects ref
}
