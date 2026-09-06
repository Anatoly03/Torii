import type { Record } from '@/types.d.ts';
import { invoke } from '@tauri-apps/api/core';

/**
 * @todo
 */
export function listRecordComponents(record: Record): Promise<any[]> {
    return invoke('list_record_components', {
        record,
    });
}

/**
 * @todo
 */
export function getRecordComponent<T extends Uint8Array | string>(record: Record, component: string): Promise<T> {
    return invoke<T>('get_record_component', {
        record,
        component,
    });
}

/**
 * @todo
 */
export function saveRecordComponent(
    record: Record,
    component: string,
    content: string,
    contentType: string
): Promise<void> {
    return invoke('save_record_component', {
        record,
        component,
        content,
        contentType,
    });
}

/**
 * @todo
 */
export function saveRecordComponentFromLocalPath(record: Record, component: string, source: string): Promise<void> {
    return invoke('save_record_component', {
        record,
        component,
        source,
    });
}

/**
 * @todo
 */
export function removeRecordComponent(record: Record, component: string): Promise<void> {
    return invoke('remove_record_component', {
        record,
        component,
    });
}

/**
 * @todo
 */
export function renameRecord(record: Record, newName: string): Promise<Record> {
    return invoke<Record>('rename_record', {
        record,
        newName,
    });
}

/**
 * @todo
 */
export function removeRecord(record: Record): Promise<void> {
    return invoke('remove_record', { record });
}

/**
 * @todo
 */
export function listRecords(workspace: string, directory: string, recursive: boolean): Promise<Record[]> {
    return invoke<Record[]>('list_records', {
        workspace,
        directory,
        recursive,
        filter: null,
    });
}

/**
 * @todo
 */
export function filterRecords(
    workspace: string,
    directory: string,
    recursive: boolean,
    filter: string | undefined
): Promise<Record[]> {
    return invoke<Record[]>('list_records', {
        workspace,
        directory,
        recursive,
        filter,
    });
}
