<script lang="ts">
  import { marked } from 'marked';
  import DOMPurify from 'dompurify';

  let { content = '' } = $props<{ content: string }>();
  
  // Parse and purify reactively
  let rawHtml = $derived(marked.parse(content, { gfm: true }) as string);
  let html = $derived(DOMPurify.sanitize(rawHtml));
</script>

<div class="markdown-body">
  {@html html}
</div>
