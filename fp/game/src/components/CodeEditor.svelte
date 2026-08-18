<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { EditorState } from '@codemirror/state';
  import { EditorView, keymap } from '@codemirror/view';
  import { defaultKeymap } from '@codemirror/commands';
  import { oneDark } from '@codemirror/theme-one-dark';
  import { python } from '@codemirror/lang-python';
  import { rust } from '@codemirror/lang-rust';
  import { javascript } from '@codemirror/lang-javascript';

  let { value = '', language = 'python', readonly = false, onchange } = $props<{
    value?: string;
    language?: string;
    readonly?: boolean;
    onchange?: (val: string) => void;
  }>();

  let container: HTMLDivElement;
  let view: EditorView;

  onMount(() => {
    const extensions = [
      keymap.of(defaultKeymap),
      oneDark,
      EditorView.updateListener.of((update) => {
        if (update.docChanged && onchange) {
          onchange(update.state.doc.toString());
        }
      })
    ];

    // Mỗi ngôn ngữ trong giáo trình có mode highlight riêng.
    // Trước đây chỉ Python được wire — Rust và TypeScript hiển thị như text thuần.
    const langMode = {
      python: python,
      rust: rust,
      typescript: () => javascript({ typescript: true }),
      javascript: () => javascript(),
    }[language];
    if (langMode) extensions.push(langMode());
    if (readonly) extensions.push(EditorState.readOnly.of(true));

    const state = EditorState.create({
      doc: value,
      extensions
    });

    view = new EditorView({
      state,
      parent: container
    });
  });

  onDestroy(() => {
    if (view) view.destroy();
  });
  
  $effect(() => {
    if (view && value !== view.state.doc.toString()) {
       view.dispatch({
         changes: { from: 0, to: view.state.doc.length, insert: value }
       });
    }
  });
</script>

<div bind:this={container} class="code-editor-wrapper"></div>

<style>
  .code-editor-wrapper {
    height: 100%;
    width: 100%;
    overflow: hidden;
  }
  .code-editor-wrapper :global(.cm-editor) {
    height: 100%;
    font-size: 14px;
  }
</style>
