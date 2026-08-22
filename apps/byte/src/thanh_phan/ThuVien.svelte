<script lang="ts">
  /** Thư viện TIER-C — 159 chương sách ở chế độ đọc.
   *
   *  Không chấm, không cấp mastery ở mức bài. Nó có mặt vì một lý do cụ thể:
   *  học liệu tương tác mới phủ được Realm 0–2, mà người vào tìm hiểu DDD hay
   *  parser combinator thì phải đọc được cái gì đó NGAY. Một lĩnh vực trống
   *  trơn nói với họ rằng chỗ này không dành cho họ.
   */
  import RichText from './RichText.svelte';
  import Byte from './Byte.svelte';
  import { thu_vien, chuong, type Chuong } from '../lib/noi_dung';

  let { dong }: { dong: () => void } = $props();

  let ds = $state<Chuong[]>([]);
  let loi = $state<string | null>(null);
  let dang_doc = $state<(Chuong & { body: unknown }) | null>(null);
  let ngon_ngu = $state<string>('python');

  $effect(() => {
    thu_vien()
      .then((c) => (ds = c))
      .catch((e: Error) => (loi = e.message));
  });

  const NGON_NGU: Record<string, string> = {
    python: 'Python',
    rust: 'Rust',
    typescript: 'TypeScript',
  };

  const theo_phan = $derived.by(() => {
    const m = new Map<string, Chuong[]>();
    for (const c of ds.filter((x) => x.lang === ngon_ngu)) {
      const cu = m.get(c.part);
      if (cu) cu.push(c);
      else m.set(c.part, [c]);
    }
    return [...m.entries()];
  });

  const ten_phan = (s: string) =>
    s.replace(/^part-\d+-/, '').replace(/-/g, ' ');

  async function mo(id: string) {
    try {
      dang_doc = await chuong(id);
      window.scrollTo({ top: 0 });
    } catch (e) {
      loi = (e as Error).message;
    }
  }
</script>

{#if loi}
  <div class="loi">
    <Byte tam_trang="to-mo" co={56} loi_thoai="Byte chưa mở được thư viện." />
    <p><code>{loi}</code></p>
    <p class="mach">Chạy <code>pnpm --filter @byte/app thu-vien</code> để sinh nội dung.</p>
  </div>

{:else if dang_doc}
  <article>
    <button class="quay-lai" onclick={() => (dang_doc = null)}>← Thư viện</button>
    <h1>{dang_doc.title}</h1>
    <p class="nguon">
      {NGON_NGU[dang_doc.lang] ?? dang_doc.lang} · {ten_phan(dang_doc.part)}
      {#if dang_doc.estimatedMinutes} · khoảng {dang_doc.estimatedMinutes} phút{/if}
    </p>
    <div class="canh-bao">
      Chương đọc — không có bài tập và không tính vào tiến độ. Nó ở đây để lĩnh
      vực này không trống trong lúc bài tương tác đang được viết.
    </div>
    <RichText noi_dung={dang_doc.body as never} />
  </article>

{:else}
  <header class="dau">
    <button class="quay-lai" onclick={dong}>← Bản đồ</button>
    <h1>Thư viện</h1>
    <p class="tom">
      {ds.length} chương ở chế độ đọc. Chúng chưa phải bài tương tác — nhưng
      phủ được những lĩnh vực mà bài tương tác chưa với tới.
    </p>
  </header>

  <div class="chon">
    {#each Object.entries(NGON_NGU) as [ma, ten] (ma)}
      <button class="tab" class:dang={ngon_ngu === ma} onclick={() => (ngon_ngu = ma)}>
        {ten}
        <span class="dem">{ds.filter((c) => c.lang === ma).length}</span>
      </button>
    {/each}
  </div>

  {#each theo_phan as [phan, cac] (phan)}
    <section>
      <h2>{ten_phan(phan)}</h2>
      <ul>
        {#each cac as c (c.id)}
          <li>
            <button class="the" onclick={() => mo(c.id)}>
              <span class="noi">
                <strong>{c.title}</strong>
                {#if c.summary}<span class="tom-c">{c.summary}</span>{/if}
              </span>
              <span class="phut">{c.estimatedMinutes ?? Math.round(c.lines / 40)}′</span>
            </button>
          </li>
        {/each}
      </ul>
    </section>
  {/each}
{/if}

<style>
  .dau { margin-bottom: 1.6rem; }
  h1 { font-size: 1.7rem; margin: 0.5rem 0 0.4rem; }
  h2 { font-size: 0.8rem; letter-spacing: 0.09em; text-transform: uppercase;
       color: var(--chu-mo); margin: 2rem 0 0.7rem; font-weight: 600; }
  .tom { color: var(--chu-mo); line-height: 1.6; margin: 0; max-width: 46ch; }
  .nguon { color: var(--chu-mo); font-size: 0.88rem; margin: 0 0 1rem; }
  .canh-bao {
    background: var(--nen-luu-y); border-left: 3px solid var(--luu-y);
    padding: 0.8rem 1rem; border-radius: 8px; font-size: 0.88rem;
    line-height: 1.6; margin-bottom: 1.8rem;
  }
  .chon { display: flex; gap: 0.5rem; margin-bottom: 0.5rem; flex-wrap: wrap; }
  .tab {
    background: var(--nen-o); border: 1.5px solid var(--vien); border-radius: 999px;
    padding: 0.4rem 1rem; font: inherit; font-size: 0.9rem; color: var(--chu-mo); cursor: pointer;
  }
  .tab.dang { border-color: var(--nhan); color: var(--nhan); }
  .tab .dem { font-variant-numeric: tabular-nums; opacity: 0.7; margin-left: 0.3rem; }
  ul { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 0.45rem; }
  .the {
    width: 100%; display: flex; align-items: center; gap: 0.9rem; text-align: left;
    background: var(--nen-o); border: 1.5px solid var(--vien); border-radius: 12px;
    padding: 0.75rem 1rem; font: inherit; color: inherit; cursor: pointer;
  }
  .the:hover { border-color: var(--nhan); }
  .noi { flex: 1; display: flex; flex-direction: column; gap: 0.12rem; }
  .tom-c { font-size: 0.85rem; color: var(--chu-mo); line-height: 1.5; }
  .phut { font-size: 0.8rem; color: var(--chu-mo); font-variant-numeric: tabular-nums; }
  .quay-lai { background: none; border: none; color: var(--chu-mo); font: inherit;
              font-size: 0.88rem; cursor: pointer; padding: 0; }
  .quay-lai:hover { color: var(--nhan); }
  .loi { display: flex; flex-direction: column; gap: 0.8rem; }
  .mach { font-size: 0.9rem; color: var(--chu-mo); }
</style>
