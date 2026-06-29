import { Extension, Editor, Content } from '@tiptap/core';
import { EditorState, Plugin, PluginKey } from 'prosemirror-state';
import { Node } from '@tiptap/pm/model';
import { EditorView } from 'prosemirror-view';

/**
 * A suggestion item for the autocomplete.
 */
export type SuggestionItem = {
    /**
     * The label for the suggestion (name of the record).
     */
    label: string;
};

export type AutocompleteOptions = {
    /**
     * Called with the current text before cursor if a suggestion is needed.
     * @param text
     * @returns A promise that resolves to an array of suggestions or null/empty
     * if none.
     */
    suggest: (text: string) => Promise<SuggestionItem[] | null>;

    /**
     * Generate the string sequence that will be inserted into the editor when a
     * suggestion is selected.
     * @param selectedItem The suggestion item that was selected.
     * @returns Raw markdown string to insert into the editor.
     */
    replace: (selectedItem: SuggestionItem) => Content | Node;

    /**
     *
     * @param text
     * @returns
     * @example
     *
     * {
     *   // alphanumeric
     *   extractToken: (text) => text.match(/(\w+)$/).?[1],
     *   // alphanumeric with underscores and dots
     *   extractToken: (text) => text.match(/[a-zA-Z0-9_.]+$/).?[1],
     * }
     */
    extractToken?: (text: string) => string | null;
};

export const ProsemirrorAutocompleteExtension = (
    options: AutocompleteOptions,
    editor: Editor
) =>
    new Plugin({
        key: new PluginKey('autocomplete'),
        view: (editorView) => {
            return {
                update: (_editorView: EditorView, _prevState: EditorState) => {
                    // TODO
                },
                destroy: () => {
                    // TODO
                },
            };
        },
    });

export const AutocompleteExtension = Extension.create<AutocompleteOptions>({
    name: 'autocomplete',

    addOptions() {
        return {
            suggest: async () => {
                console.warn('Autocomplete suggest function not set.');
                return [];
            },
            replace: (item) => item.label,
            extractToken: (text) => {
                const match = text.match(/(\w+)$/);
                return match ? match[1] : null;
            },
        };
    },

    addProseMirrorPlugins() {
        return [ProsemirrorAutocompleteExtension(this.options, this.editor)];
    },
});

export default AutocompleteExtension;
