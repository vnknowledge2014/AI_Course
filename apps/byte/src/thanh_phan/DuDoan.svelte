<script lang="ts">
  import RichText from './RichText.svelte';
  import Byte from './Byte.svelte';
  import type { PredictStep } from '@byte/content-schema';

  let { buoc, xong }: { buoc: PredictStep; xong: () => void } = $props();

  let da_chon = $state<string | null>(null);

  const cac_lua_chon = $derived(
    buoc.answer.kind === 'choice' && 'options' in buoc.answer
      ? (buoc.answer as { options: { id: string; label: unknown; correct?: boolean; why?: unknown }[] })
          .options
      : [],
  );
  const chon = $derived(cac_lua_chon.find((o) => o.id === da_chon) ?? null);
  const dung = $derived(chon?.correct === true);

  function bam(id: string) {
    // `commitOnce`: một khi đã cam kết thì không đổi được nữa. Đó chính là
    // điều làm bước này có giá trị — đoán rồi mới thấy kết quả thì mới nhớ;
    // đổi đáp án sau khi biết đúng sai thì không học được gì.
    if (da_chon !== null) return;
    da_chon = id;
  }
</script>

<div class="du-doan">
  <RichText noi_dung={buoc.body} />

  <ul class="cac-o">
    {#each cac_lua_chon as o (o.id)}
      <li>
        <button
          class="o"
          class:da-chon={da_chon === o.id}
          class:dung={da_chon !== null && o.correct === true}
          class:sai={da_chon === o.id && o.correct !== true}
          disabled={da_chon !== null}
          onclick={() => bam(o.id)}
        >
          <RichText noi_dung={o.label as never} />
        </button>
      </li>
    {/each}
  </ul>

  {#if chon}
    <div class="phan-hoi">
      {#if dung}
        <Byte tam_trang="reo-len" co={52} loi_thoai="Chuẩn. Bạn đoán trước rồi mới chạy — đó là thói quen của người viết code giỏi." />
      {:else}
        <Byte tam_trang="to-mo" co={52} loi_thoai="Đoán khác một chút. Xem thử vì sao nhé — chỗ này mới là chỗ học được nhiều nhất." />
        {#if chon.why}
          <div class="vi-sao"><RichText noi_dung={chon.why as never} /></div>
        {/if}
      {/if}
      <button class="tiep" onclick={xong}>Đi tiếp</button>
    </div>
  {/if}
</div>

<style>
  .cac-o { list-style: none; padding: 0; margin: 1.2rem 0 0; display: flex; flex-direction: column; gap: 0.6rem; }
  .o {
    width: 100%;
    text-align: left;
    background: var(--nen-o);
    border: 1.5px solid var(--vien);
    border-radius: 12px;
    padding: 0.85rem 1.05rem;
    font: inherit;
    color: inherit;
    cursor: pointer;
  }
  .o:hover:not(:disabled) { border-color: var(--nhan); }
  .o:disabled { cursor: default; }
  .o.dung { border-color: var(--dung); background: var(--nen-dung); }
  .o.sai { border-color: var(--luu-y); background: var(--nen-luu-y); }
  .phan-hoi { margin-top: 1.4rem; display: flex; flex-direction: column; gap: 1rem; }
  .vi-sao {
    background: var(--nen-o);
    border-radius: 12px;
    padding: 1.05rem 1.2rem;
    border-left: 3px solid var(--luu-y);
  }
  .tiep {
    align-self: flex-start;
    background: var(--nhan);
    color: var(--nen);
    border: none;
    border-radius: 999px;
    padding: 0.6rem 1.4rem;
    font: inherit;
    font-weight: 600;
    cursor: pointer;
  }
</style>
