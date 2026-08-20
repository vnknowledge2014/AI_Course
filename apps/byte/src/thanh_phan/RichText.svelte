<script lang="ts">
  import { ket_xuat } from '@byte/ui';
  import type { RichText } from '@byte/content-schema';

  let { noi_dung }: { noi_dung: RichText } = $props();

  // `ket_xuat` thoát HTML nghiêm ngặt ở tầng gốc (@byte/ui/richtext.ts) — nội
  // dung bài học không bao giờ được phép chèn thẻ. Đó là lý do dùng {@html}
  // ở đây là an toàn, và cũng là lý do KHÔNG được thay bằng một bộ dựng khác
  // mà không kiểm lại chuyện thoát ký tự.
  const html = $derived(ket_xuat(noi_dung));
</script>

<div class="van-ban">{@html html}</div>

<style>
  .van-ban :global(p) { margin: 0 0 0.9em; line-height: 1.75; }
  .van-ban :global(p:last-child) { margin-bottom: 0; }
  .van-ban :global(code) {
    font-family: var(--font-ma);
    font-size: 0.92em;
    background: var(--nen-ma);
    padding: 0.12em 0.38em;
    border-radius: 4px;
  }
  .van-ban :global(pre) {
    background: var(--nen-ma);
    padding: 1rem 1.1rem;
    border-radius: 10px;
    overflow-x: auto;
    line-height: 1.6;
  }
  .van-ban :global(pre code) { background: none; padding: 0; }
  .van-ban :global(ul), .van-ban :global(ol) { padding-left: 1.4rem; line-height: 1.75; }
  .van-ban :global(li) { margin-bottom: 0.4em; }
  .van-ban :global(strong) { font-weight: 650; color: var(--chu-dam); }
  .van-ban :global(a) { color: var(--nhan); }
  /* Bảng rộng cuộn trong khung của nó, không đẩy cả trang trượt ngang. */
  .van-ban :global(.bang-cuon) { overflow-x: auto; }
</style>
