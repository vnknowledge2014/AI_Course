<script lang="ts">
  import RichText from './RichText.svelte';
  import type { HintLadder } from '@byte/content-schema';

  let { thang }: { thang: HintLadder } = $props();

  // Thang gợi ý mở TỪNG NẤC MỘT, không mở hết cùng lúc.
  //
  // Nấc 1 hướng mắt, nấc 2 hướng cách nghĩ, nấc 3 mới đưa câu trả lời. Cho
  // xem cả ba ngay lập tức thì mọi người sẽ đọc thẳng nấc cuối, và bài học
  // biến thành bài chép. Bắt bấm thêm một lần cho mỗi nấc là đủ ma sát để
  // người ta thử nghĩ trước.
  let da_mo = $state(0);

  const nhan: Record<string, string> = {
    attention: 'Nhìn vào đâu',
    strategy: 'Nghĩ theo hướng nào',
    'one-line': 'Câu trả lời',
    solution: 'Lời giải đầy đủ',
  };
  const con_lai = $derived(thang.rungs.length - da_mo);
</script>

<div class="goi-y">
  {#each thang.rungs.slice(0, da_mo) as nac (nac.kind)}
    <div class="nac">
      <span class="nhan">{nhan[nac.kind] ?? nac.kind}</span>
      <RichText noi_dung={nac.body} />
    </div>
  {/each}

  {#if con_lai > 0}
    <button class="mo" onclick={() => (da_mo += 1)}>
      {da_mo === 0 ? 'Bí rồi, gợi ý cho mình' : `Vẫn chưa ra — gợi ý rõ hơn (còn ${con_lai})`}
    </button>
  {/if}
</div>

<style>
  .goi-y { margin-top: 1.25rem; display: flex; flex-direction: column; gap: 0.75rem; }
  .nac {
    border-left: 3px solid var(--nhan);
    padding: 0.1rem 0 0.1rem 0.9rem;
  }
  .nhan {
    display: block;
    font-size: 0.76rem;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--chu-mo);
    margin-bottom: 0.3rem;
  }
  .mo {
    align-self: flex-start;
    background: none;
    border: 1px solid var(--vien);
    color: var(--chu-nhat);
    padding: 0.5rem 0.95rem;
    border-radius: 999px;
    font: inherit;
    font-size: 0.9rem;
    cursor: pointer;
  }
  .mo:hover { border-color: var(--nhan); color: var(--nhan); }
</style>
