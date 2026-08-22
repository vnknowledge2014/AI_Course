<script lang="ts">
  /** Sân khấu `number-line` — phát lại một lần chạy trên thanh số.
   *
   *  Ba bài của T2.1 dạy đúng thứ chỉ NHÌN mới hiểu: cộng là bước sang phải,
   *  nhân là kéo giãn cả thanh, nhân số âm là lật quanh mốc 0. Câu
   *  "(−1) × (−1) = 1" đọc thì phải tin; thấy thanh lật hai lần rồi về chỗ cũ
   *  thì không phải tin nữa.
   */
  import type { SuKien, CauHinhThanhSo } from './kieu';

  let {
    thanh,
    su_kien = [],
    thang = false,
  }: { thanh: CauHinhThanhSo; su_kien?: SuKien[]; thang?: boolean } = $props();

  const W = 560;
  const H = 96;
  const LE = 26;

  let buoc = $state(0);
  let dang_phat = $state(false);
  let dau_van = $state('');

  $effect(() => {
    const van = su_kien.map((s) => s.t).join(',');
    if (van !== dau_van) {
      dau_van = van;
      buoc = 0;
      if (su_kien.length > 0) phat();
    }
  });

  function phat() {
    if (dang_phat) return;
    dang_phat = true;
    const nhip = setInterval(() => {
      if (buoc >= su_kien.length) {
        clearInterval(nhip);
        dang_phat = false;
        return;
      }
      buoc += 1;
    }, 420);
  }

  const so = (p: Record<string, unknown>, k: string, md: number): number => {
    const v = p[k];
    return typeof v === 'number' ? v : md;
  };

  /** Trạng thái sau `buoc` sự kiện. */
  const canh = $derived.by(() => {
    let o = thanh.bat_dau;
    let loai: 'di' | 'nhan' | null = null;
    let tu = o;
    let he_so = 1;
    let noi: string | null = null;
    let ra_ngoai = false;

    for (const s of su_kien.slice(0, buoc)) {
      const p = s.payload;
      if (s.kind === 'move') { tu = so(p, 'tu', o); o = so(p, 'den', o); loai = 'di'; }
      else if (s.kind === 'borrow') {
        tu = so(p, 'tu', o); o = so(p, 'den', o); he_so = so(p, 'he_so', 1); loai = 'nhan';
      } else if (s.kind === 'print') noi = String(p['cau'] ?? '');
      else if (s.kind === 'panic') ra_ngoai = true;
    }
    return { o, loai, tu, he_so, noi, ra_ngoai };
  });

  const toa_do = (v: number) =>
    LE + ((v - thanh.tu) / (thanh.den - thanh.tu)) * (W - 2 * LE);

  const buoc_vach = $derived(thanh.buoc_vach ?? Math.max(1, Math.round((thanh.den - thanh.tu) / 20)));
  const vach = $derived.by(() => {
    const r: number[] = [];
    for (let v = Math.ceil(thanh.tu / buoc_vach) * buoc_vach; v <= thanh.den; v += buoc_vach) r.push(v);
    return r;
  });

  const nhan_so = (v: number) => (Number.isInteger(v) ? String(v) : String(Math.round(v * 1e4) / 1e4));
  const xong = $derived(buoc >= su_kien.length && su_kien.length > 0);
</script>

<div class="san">
  <svg viewBox="0 0 {W} {H}" width={W} height={H} role="img"
       aria-label="Thanh số từ {thanh.tu} tới {thanh.den}, đang ở {nhan_so(canh.o)}">
    <line class="truc" x1={LE} y1={H / 2} x2={W - LE} y2={H / 2} />

    {#each vach as v (v)}
      <line class="vach" class:goc={v === 0} x1={toa_do(v)} y1={H / 2 - 6} x2={toa_do(v)} y2={H / 2 + 6} />
      <text class="nhan" x={toa_do(v)} y={H / 2 + 24}>{nhan_so(v)}</text>
    {/each}

    <!-- Đích: hiện suốt để người học biết mình đang nhắm vào đâu. -->
    {#if thanh.dich !== undefined}
      <circle class="dich" cx={toa_do(thanh.dich)} cy={H / 2} r="9" />
    {/if}

    <!-- Cung nối chỗ cũ với chỗ mới: đó là thứ làm "bước" và "kéo giãn" nhìn
         ra khác nhau, chứ không phải chỉ thấy chấm nhảy chỗ. -->
    {#if canh.loai && canh.tu !== canh.o}
      <path
        class="cung"
        class:keo={canh.loai === 'nhan'}
        d="M {toa_do(canh.tu)} {H / 2 - 4} Q {(toa_do(canh.tu) + toa_do(canh.o)) / 2} {H / 2 - 34} {toa_do(canh.o)} {H / 2 - 4}"
      />
    {/if}

    <circle class="cho-dung" class:ra-ngoai={canh.ra_ngoai} cx={toa_do(canh.o)} cy={H / 2} r="7" />
    <text class="cho-nhan" x={toa_do(canh.o)} y={H / 2 - 16}>{nhan_so(canh.o)}</text>
  </svg>

  <div class="duoi">
    <span class="dem">Bước {buoc}/{su_kien.length}</span>
    {#if canh.loai === 'nhan'}<span class="phep">nhân {nhan_so(canh.he_so)} — {canh.he_so < 0 ? 'lật' : 'kéo giãn'}</span>{/if}
    {#if canh.noi}<span class="noi">Byte: “{canh.noi}”</span>{/if}
    {#if xong && thanh.dich !== undefined}
      <span class="ket" class:thang class:truot={!thang}>{thang ? 'Tới đích.' : 'Chưa tới đích.'}</span>
    {/if}
    {#if su_kien.length > 0 && !dang_phat}
      <button class="lai" onclick={() => { buoc = 0; phat(); }}>Xem lại</button>
    {/if}
  </div>
</div>

<style>
  .san { display: flex; flex-direction: column; gap: 0.6rem; margin: 1.2rem 0; }
  svg { background: var(--nen-ma); border-radius: 10px; max-width: 100%; height: auto; }
  .truc { stroke: var(--chu-mo); stroke-width: 1.5; }
  .vach { stroke: var(--vien); stroke-width: 1.5; }
  .vach.goc { stroke: var(--chu-mo); stroke-width: 2.5; }
  .nhan { fill: var(--chu-mo); font-size: 11px; text-anchor: middle; font-variant-numeric: tabular-nums; }
  .dich { fill: none; stroke: var(--dung); stroke-width: 2.5; stroke-dasharray: 3 3; }
  .cung { fill: none; stroke: var(--nhan); stroke-width: 2; }
  .cung.keo { stroke-dasharray: 5 3; }
  .cho-dung { fill: var(--byte-vien); transition: cx 0.35s ease; }
  .cho-dung.ra-ngoai { fill: var(--luu-y); }
  .cho-nhan { fill: var(--chu-dam); font-size: 12px; font-weight: 600; text-anchor: middle;
              font-variant-numeric: tabular-nums; }
  @media (prefers-reduced-motion: reduce) { .cho-dung { transition: none; } }
  .duoi { display: flex; align-items: center; gap: 0.9rem; flex-wrap: wrap; font-size: 0.86rem; }
  .dem { color: var(--chu-mo); font-variant-numeric: tabular-nums; }
  .phep { color: var(--nhan); }
  .ket.thang { color: var(--dung); font-weight: 600; }
  .ket.truot { color: var(--luu-y); font-weight: 600; }
  .lai { background: none; border: 1px solid var(--vien); color: var(--chu-mo);
         border-radius: 999px; padding: 0.25rem 0.8rem; font: inherit; font-size: 0.82rem; cursor: pointer; }
  .lai:hover { border-color: var(--nhan); color: var(--nhan); }
</style>
