<script lang="ts">
  /** Sân khấu `grid-bot` — phát lại chuỗi sự kiện của một lần chạy.
   *
   *  Phát LẠI chứ không chạy song song: mã người học đã chạy xong trong worker
   *  và để lại một chuỗi sự kiện. Sân khấu chỉ tua nó. Nhờ vậy vòng lặp vô hạn
   *  trong bài của họ không bao giờ chạm được vào luồng giao diện — mà giao
   *  diện lại là chỗ duy nhất còn nút Dừng.
   */
  import type { SuKien, CauHinhLuoi } from '@byte/exec-python';

  let {
    luoi,
    su_kien = [],
    thang = false,
  }: { luoi: CauHinhLuoi; su_kien?: SuKien[]; thang?: boolean } = $props();

  const O = 44;

  let buoc = $state(0);
  let dang_phat = $state(false);

  // Tua lại từ đầu mỗi khi có lần chạy mới.
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
    }, 260);
  }

  /** Trạng thái thế giới sau `buoc` sự kiện đầu tiên. */
  const canh = $derived.by(() => {
    let x = luoi.bat_dau.x;
    let y = luoi.bat_dau.y;
    let huong = luoi.huong ?? 0;
    const nhat = new Set<string>();
    let dam: { x: number; y: number } | null = null;
    let noi: string | null = null;

    // `noUncheckedIndexedAccess` khiến tra một `Record` cho ra `| undefined`,
    // và luật ấy đúng: payload đến từ Python nên không có gì bảo đảm khoá nào
    // có mặt. Đọc qua hai hàm nhỏ này thay vì ép kiểu — ép kiểu chỉ giấu đi
    // đúng cái rủi ro mà luật đang chỉ ra.
    const so = (p: Record<string, unknown>, k: string, mac_dinh: number): number => {
      const v = p[k];
      return typeof v === 'number' ? v : mac_dinh;
    };
    const chu = (p: Record<string, unknown>, k: string): string => String(p[k] ?? '');

    for (const s of su_kien.slice(0, buoc)) {
      const p = s.payload;
      if (s.kind === 'move') { x = so(p, 'x', x); y = so(p, 'y', y); }
      else if (s.kind === 'assign') huong = so(p, 'huong', huong);
      else if (s.kind === 'retrieve') nhat.add(`${so(p, 'x', -1)},${so(p, 'y', -1)}`);
      else if (s.kind === 'panic') dam = { x: so(p, 'x', -1), y: so(p, 'y', -1) };
      else if (s.kind === 'print') noi = chu(p, 'cau');
    }
    return { x, y, huong, nhat, dam, noi };
  });

  const con_lai = $derived((luoi.vien ?? []).filter((v) => !canh.nhat.has(`${v.x},${v.y}`)));
  const xong = $derived(buoc >= su_kien.length && su_kien.length > 0);
</script>

<div class="san-khau">
  <svg
    width={luoi.rong * O}
    height={luoi.cao * O}
    viewBox="0 0 {luoi.rong * O} {luoi.cao * O}"
    role="img"
    aria-label="Lưới {luoi.rong}×{luoi.cao}, Byte đang ở ô {canh.x},{canh.y}"
  >
    {#each Array(luoi.cao) as _, hang}
      {#each Array(luoi.rong) as _, cot}
        <rect class="o" x={cot * O} y={hang * O} width={O} height={O} rx="4" />
      {/each}
    {/each}

    {#each luoi.tuong ?? [] as t (`${t.x},${t.y}`)}
      <rect class="tuong" x={t.x * O + 2} y={t.y * O + 2} width={O - 4} height={O - 4} rx="5" />
    {/each}

    {#each con_lai as v (`${v.x},${v.y}`)}
      <circle class="vien" cx={v.x * O + O / 2} cy={v.y * O + O / 2} r={O * 0.17} />
    {/each}

    {#if canh.dam}
      <rect class="dam" x={canh.dam.x * O + 2} y={canh.dam.y * O + 2} width={O - 4} height={O - 4} rx="5" />
    {/if}

    <g
      class="byte"
      style="transform: translate({canh.x * O + O / 2}px, {canh.y * O + O / 2}px) rotate({canh.huong * 90}deg)"
    >
      <rect class="than" x={-O * 0.3} y={-O * 0.3} width={O * 0.6} height={O * 0.6} rx={O * 0.16} />
      <!-- Mũi chỉ hướng: người học phải nhìn ra Byte đang quay về đâu TRƯỚC khi
           bấm chạy, nếu không thì `quay_phai()` chỉ là một từ. -->
      <path class="mui" d="M {O * 0.16} 0 L {O * 0.02} {-O * 0.1} L {O * 0.02} {O * 0.1} Z" />
    </g>
  </svg>

  <div class="duoi">
    <span class="dem">Bước {buoc}/{su_kien.length}</span>
    {#if canh.noi}<span class="noi">Byte: “{canh.noi}”</span>{/if}
    {#if xong}
      <span class="ket" class:thang class:truot={!thang}>
        {thang ? 'Nhặt hết rồi.' : canh.dam ? 'Đâm vào tường.' : `Còn ${con_lai.length} viên.`}
      </span>
    {/if}
    {#if su_kien.length > 0 && !dang_phat}
      <button class="lai" onclick={() => { buoc = 0; phat(); }}>Xem lại</button>
    {/if}
  </div>
</div>

<style>
  .san-khau { display: flex; flex-direction: column; gap: 0.7rem; align-items: flex-start; margin: 1.2rem 0; }
  svg { background: var(--nen-ma); border-radius: 10px; max-width: 100%; height: auto; }
  .o { fill: none; stroke: var(--vien); stroke-width: 1; }
  .tuong { fill: var(--vien); }
  .vien { fill: var(--luu-y); }
  .dam { fill: none; stroke: var(--luu-y); stroke-width: 2.5; }
  .than { fill: var(--byte-than); stroke: var(--byte-vien); stroke-width: 2; }
  .mui { fill: var(--byte-vien); }
  .byte { transition: transform 0.22s ease; }
  @media (prefers-reduced-motion: reduce) { .byte { transition: none; } }
  .duoi { display: flex; align-items: center; gap: 0.9rem; flex-wrap: wrap; font-size: 0.86rem; }
  .dem { color: var(--chu-mo); font-variant-numeric: tabular-nums; }
  .noi { color: var(--chu-nhat); }
  .ket.thang { color: var(--dung); font-weight: 600; }
  .ket.truot { color: var(--luu-y); font-weight: 600; }
  .lai { background: none; border: 1px solid var(--vien); color: var(--chu-mo);
         border-radius: 999px; padding: 0.25rem 0.8rem; font: inherit; font-size: 0.82rem; cursor: pointer; }
  .lai:hover { border-color: var(--nhan); color: var(--nhan); }
</style>
