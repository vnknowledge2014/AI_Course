<script lang="ts">
  import Byte from './Byte.svelte';
  import type { RealmTrenBanDo, BaiTrenBanDo } from '../lib/cay_ky_nang';

  let {
    ban_do,
    so_ky_nang,
    ke_tiep,
    mo,
  }: {
    ban_do: RealmTrenBanDo[];
    so_ky_nang: number;
    ke_tiep: BaiTrenBanDo | null;
    mo: (id: string) => void;
  } = $props();

  const tong_xong = $derived(ban_do.reduce((s, r) => s + r.xong, 0));
  const tong_bai = $derived(ban_do.reduce((s, r) => s + r.tong, 0));

  const loi_byte = $derived(
    tong_xong === 0
      ? 'Chào bạn. Mình là Byte. Bắt đầu từ bài đầu tiên nhé — chậm mà chắc.'
      : ke_tiep
        ? `Bạn đã thạo ${so_ky_nang} kỹ năng. Bài tiếp theo đang mở.`
        : 'Bạn đã đi hết những bài đang mở. Nội dung mới sẽ nối vào đây.',
  );

  const ten_track = (s: string) => s.replace(/-/g, ' ');
</script>

<header class="dau">
  <Byte tam_trang={tong_xong === 0 ? 'vui' : 'to-mo'} co={84} loi_thoai={loi_byte} />
  <div class="so-lieu">
    <div><b>{so_ky_nang}</b><span>kỹ năng</span></div>
    <div><b>{tong_xong}/{tong_bai}</b><span>bài</span></div>
  </div>
</header>

{#if ke_tiep}
  <button class="tiep-tuc" onclick={() => mo(ke_tiep.id)}>
    <span class="nhan">Học tiếp</span>
    <strong>{ke_tiep.title}</strong>
    <span class="tom">{ke_tiep.summary}</span>
  </button>
{/if}

{#each ban_do as r (r.id)}
  <section class="realm" class:khoa={!r.mo}>
    <h2>
      {r.ten}
      <span class="dem">{r.xong}/{r.tong}</span>
    </h2>

    {#each r.track as t (t.id)}
      <div class="track">
        <h3>{ten_track(t.id)} <span class="dem">{t.xong}/{t.tong}</span></h3>

        {#if t.tong === 0}
          <p class="chua-viet">Chưa viết — mạch đã thiết kế xong.</p>
        {:else}
          <ol class="chuoi">
            {#each t.bai as b (b.id)}
              <li>
                <button
                  class="nut"
                  class:xong={b.xong}
                  class:mo={b.mo && !b.xong}
                  class:khoa={!b.mo}
                  disabled={!b.mo}
                  onclick={() => mo(b.id)}
                  title={b.mo
                    ? b.title
                    : `Cần học trước: ${b.con_thieu.map((c) => c.hoc_o ?? c.skill).join(', ')}`}
                >
                  {#if b.xong}✓{:else if b.mo}{b.order}{:else}🔒{/if}
                </button>
              </li>
            {/each}
          </ol>

          <ul class="ds">
            {#each t.bai.filter((b) => b.mo) as b (b.id)}
              <li>
                <button class="the" class:xong={b.xong} onclick={() => mo(b.id)}>
                  <span class="so">{b.order}</span>
                  <span class="noi">
                    <strong>{b.title}</strong>
                    <span class="tom">{b.summary}</span>
                  </span>
                  <span class="phut">{b.estimatedMinutes}′</span>
                  {#if b.xong}<span class="dau-xong">✓</span>{/if}
                </button>
              </li>
            {/each}
          </ul>
        {/if}
      </div>
    {/each}
  </section>
{/each}

<style>
  .dau { display: flex; align-items: flex-start; justify-content: space-between; gap: 1.5rem; margin-bottom: 1.6rem; flex-wrap: wrap; }
  .so-lieu { display: flex; gap: 1.6rem; }
  .so-lieu div { display: flex; flex-direction: column; align-items: flex-end; }
  .so-lieu b { font-size: 1.5rem; font-variant-numeric: tabular-nums; line-height: 1.2; }
  .so-lieu span { font-size: 0.76rem; letter-spacing: 0.06em; text-transform: uppercase; color: var(--chu-mo); }

  .tiep-tuc {
    display: flex; flex-direction: column; gap: 0.2rem; width: 100%; text-align: left;
    background: var(--nen-o); border: 1.5px solid var(--nhan); border-radius: 14px;
    padding: 1rem 1.2rem; font: inherit; color: inherit; cursor: pointer; margin-bottom: 2rem;
  }
  .tiep-tuc .nhan { font-size: 0.74rem; letter-spacing: 0.08em; text-transform: uppercase; color: var(--nhan); }
  .tiep-tuc strong { font-size: 1.1rem; }
  .tiep-tuc .tom { font-size: 0.88rem; color: var(--chu-mo); line-height: 1.55; }

  .realm { margin-bottom: 2.6rem; }
  .realm.khoa { opacity: 0.55; }
  h2 { font-size: 1.05rem; margin: 0 0 1rem; display: flex; align-items: baseline; gap: 0.6rem; }
  h3 { font-size: 0.78rem; letter-spacing: 0.08em; text-transform: uppercase; color: var(--chu-mo);
       margin: 1.4rem 0 0.6rem; font-weight: 600; display: flex; align-items: baseline; gap: 0.5rem; }
  .dem { font-size: 0.8rem; font-weight: 400; color: var(--chu-mo); font-variant-numeric: tabular-nums; }
  .chua-viet { font-size: 0.86rem; color: var(--chu-mo); margin: 0.2rem 0 0; }

  /* Chuỗi nút — cái nhìn tổng thể một track trong một dòng. */
  .chuoi { list-style: none; display: flex; flex-wrap: wrap; gap: 0.3rem; padding: 0; margin: 0 0 0.9rem; }
  .nut {
    width: 1.9rem; height: 1.9rem; border-radius: 50%; font: inherit; font-size: 0.72rem;
    display: grid; place-items: center; cursor: pointer;
    border: 1.5px solid var(--vien); background: var(--nen-o); color: var(--chu-mo);
    font-variant-numeric: tabular-nums;
  }
  .nut.mo { border-color: var(--nhan); color: var(--nhan); }
  .nut.xong { background: var(--dung); border-color: var(--dung); color: var(--nen); }
  .nut.khoa { cursor: default; opacity: 0.5; }

  .ds { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 0.45rem; }
  .the {
    width: 100%; display: flex; align-items: center; gap: 0.9rem; text-align: left;
    background: var(--nen-o); border: 1.5px solid var(--vien); border-radius: 12px;
    padding: 0.75rem 1rem; font: inherit; color: inherit; cursor: pointer;
  }
  .the:hover { border-color: var(--nhan); }
  .the.xong { opacity: 0.66; }
  .so { width: 1.7rem; text-align: center; color: var(--chu-mo); font-variant-numeric: tabular-nums; }
  .noi { flex: 1; display: flex; flex-direction: column; gap: 0.12rem; }
  .tom { font-size: 0.85rem; color: var(--chu-mo); line-height: 1.5; }
  .phut { font-size: 0.8rem; color: var(--chu-mo); font-variant-numeric: tabular-nums; }
  .dau-xong { color: var(--dung); font-weight: 700; }
</style>
