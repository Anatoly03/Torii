import { Extension, Editor, Content } from '@tiptap/core';
import { EditorState, Plugin, PluginKey } from 'prosemirror-state';
import { Node } from '@tiptap/pm/model';
import { EditorView } from 'prosemirror-view';
import { Ref } from 'vue';
import MarkdownEditorAutocomplete from './MarkdownEditorAutocomplete.vue';

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

    /**
     * A reference to the autocomplete popup component. This is used to
     * control the visibility, position and options of the popup.
     */
    popup?: Ref<InstanceType<typeof MarkdownEditorAutocomplete> | null>;
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
                    // We only apply the autocomplete logic within one element of the editor.
                    //
                    // For example `<a href="#">hel</a>lo` with the cursor at the end will autocomplete
                    // for 'lo', not 'hello'.
                    //
                    // The exception is bold, italic and underline.
                    // For example `<b>hel</>lo` with the cursor at the end will autocomplete
                    // for 'hello'.
                    //
                    // Below $nodes is computed to be the content of the current element after resolving
                    // the selection position. We only apply the autocomplete logic within this context.
                    const $from = editorView.state.selection.$from;
                    // const $parent = $from.parent;
                    // const $nodes = $parent.content.content;

                    // Find the currently selected node within the $nodes array.
                    const $nodeBefore = $from.nodeBefore;
                    const textBeforeCursor = $nodeBefore?.text ?? '';

                    // Get the suggestions for the current text before the cursor.
                    options.suggest(textBeforeCursor).then((suggestions) => {
                        // Get the list of suggestions and propagate them to the
                        // autocomplete popup component.
                        if (options.popup?.value) {
                            options.popup.value.setSuggestions(suggestions ?? []);
                            options.popup.value.show();
                        }

                        if (!suggestions || suggestions.length === 0) {
                            return;
                        }

                        console.log(options.popup?.value);
                    });

                    // const $pos = editorView.state.selection.$from.pos;
                    // const element = editorView.domAtPos($pos);

                    // console.log($nodeBefore, $nodes);
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
