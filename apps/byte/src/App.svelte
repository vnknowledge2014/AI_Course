<script lang="ts">
  import { muc_luc, bai_hoc, thu_tu, type MucLuc } from './lib/noi_dung';
  import { dung_ban_do, bai_ke_tiep, so_ky_nang } from './lib/cay_ky_nang';
  import BanDo from './thanh_phan/BanDo.svelte';
  import { doc, danh_dau_buoc, danh_dau_xong } from './lib/tien_do';
  import Buoc from './thanh_phan/Buoc.svelte';
  import Byte from './thanh_phan/Byte.svelte';
  import type { Lesson } from '@byte/content-schema';

  let ds = $state<MucLuc[]>([]);
  let bai = $state<Lesson | null>(null);
  let so_buoc_hien = $state(1);
  let tien_do = $state(doc());
  let loi = $state<string | null>(null);

  let realm = $state<ReturnType<typeof thu_tu>>([]);

  $effect(() => {
    muc_luc()
      .then((m) => {
        ds = m;
        realm = thu_tu();
      })
      .catch((e: Error) => (loi = e.message));
  });

  // Bản đồ tính lại mỗi khi tiến độ đổi — mở khoá là hàm của tiến độ, không
  // phải một trạng thái riêng phải nhớ đồng bộ.
  const ban_do = $derived(ds.length ? dung_ban_do(ds, realm, tien_do) : []);
  const ke_tiep = $derived(bai_ke_tiep(ban_do));
  const ky_nang = $derived(so_ky_nang(ds, tien_do));

  async function mo(id: string) {
    try {
      bai = await bai_hoc(id);
      // Mở lại đúng chỗ đang dở, nhưng luôn hiện ít nhất một bước.
      so_buoc_hien = Math.max(1, tien_do.buoc_xong[id] ?? 1);
    } catch (e) {
      loi = (e as Error).message;
    }
  }

  function tiep() {
    if (!bai) return;
    if (so_buoc_hien < bai.steps.length) {
      so_buoc_hien += 1;
      tien_do = danh_dau_buoc(bai.id, so_buoc_hien);
    } else {
      tien_do = danh_dau_xong(bai.id);
    }
  }

  const nhom = $derived(
    ds.reduce<Record<string, MucLuc[]>>((acc, l) => {
      (acc[l.module] ??= []).push(l);
      return acc;
    }, {}),
  );
  const het_bai = $derived(bai !== null && so_buoc_hien >= bai.steps.length);
  const da_xong_bai = $derived(bai !== null && tien_do.da_xong.includes(bai.id));
</script>

<main>
  {#if loi}
    <div class="loi">
      <Byte tam_trang="to-mo" co={56} loi_thoai="Byte chưa mở được nội dung." />
      <p><code>{loi}</code></p>
      <p class="mach">Chạy <code>pnpm --filter @byte/app noi-dung</code> để sinh nội dung trước.</p>
    </div>

  {:else if bai}
    <article>
      <header class="dau-bai">
        <button class="quay-lai" onclick={() => (bai = null)}>← Danh sách bài</button>
        <h1>{bai.title}</h1>
        <p class="tom-tat">{bai.summary}</p>
        <div class="thanh"><div class="da-di" style="width: {(so_buoc_hien / bai.steps.length) * 100}%"></div></div>
        <p class="dem">Bước {so_buoc_hien}/{bai.steps.length} · khoảng {bai.estimatedMinutes} phút</p>
      </header>

      {#each bai.steps.slice(0, so_buoc_hien) as b, i (b.id)}
        <div class="o-buoc" class:mo-nhat={i < so_buoc_hien - 1}>
          <Buoc buoc={b} xong={tiep} />
        </div>
      {/each}

      <footer>
        {#if !het_bai}
          <button class="tiep" onclick={tiep}>Tiếp tục</button>
        {:else if !da_xong_bai}
          <button class="tiep" onclick={tiep}>Xong bài này</button>
        {:else}
          <Byte tam_trang="vui" co={56} loi_thoai="Xong rồi. Bài sau nối thẳng vào câu hỏi vừa để ngỏ ở đây." />
          <button class="quay-lai" onclick={() => (bai = null)}>Chọn bài tiếp theo</button>
        {/if}
      </footer>
    </article>

  {:else}
    <BanDo {ban_do} so_ky_nang={ky_nang} {ke_tiep} mo={(id) => mo(id)} />
  {/if}
</main>

<style>
  main { max-width: 44rem; margin: 0 auto; padding: 2.5rem 1.25rem 6rem; }
  h1 { font-size: 1.85rem; line-height: 1.25; margin: 0.6rem 0 0.4rem; }
  .tom-tat { color: var(--chu-mo); margin: 0 0 1.2rem; line-height: 1.6; }
  .dem { font-size: 0.85rem; color: var(--chu-mo); margin: 0.5rem 0 0; }
  .thanh { height: 4px; background: var(--vien); border-radius: 999px; overflow: hidden; }
  .da-di { height: 100%; background: var(--nhan); transition: width 0.35s ease; }
  .dau-bai { margin-bottom: 2.2rem; }
  .o-buoc { padding: 1.6rem 0; border-bottom: 1px solid var(--vien); }
  /* Bước đã qua mờ đi nhưng KHÔNG bị ẩn: người học phải cuộn ngược lại đọc
     được, vì mạch của bài nằm ở chỗ bước sau trả lời bước trước. */
  .o-buoc.mo-nhat { opacity: 0.62; }
  footer { padding-top: 1.8rem; display: flex; flex-direction: column; gap: 1rem; align-items: flex-start; }
  .tiep {
    background: var(--nhan); color: var(--nen); border: none; border-radius: 999px;
    padding: 0.65rem 1.5rem; font: inherit; font-weight: 600; cursor: pointer;
  }
  .quay-lai {
    background: none; border: none; color: var(--chu-mo); font: inherit;
    font-size: 0.88rem; cursor: pointer; padding: 0;
  }
  .quay-lai:hover { color: var(--nhan); }
  .loi { display: flex; flex-direction: column; gap: 0.8rem; }
  .mach { font-size: 0.9rem; color: var(--chu-mo); }
</style>
