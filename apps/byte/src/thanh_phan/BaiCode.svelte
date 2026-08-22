<script lang="ts">
  import RichText from './RichText.svelte';
  import GoiY from './GoiY.svelte';
  import Byte from './Byte.svelte';
  // Đổi tên lúc import: component đã có một hàm `chay()` là handler của nút.
  import { chay as chay_theo_ngon_ngu } from '../lib/chay_ma';
  import { tab, tab_nguoc, enter, backspace } from '../lib/soan_thao';
  import SanKhau from './SanKhau.svelte';
  import ThanhSo from './ThanhSo.svelte';
  import type { CodeStep, KetQuaChay } from './kieu';

  let {
    buoc,
    xong,
    san_choi = false,
  }: { buoc: CodeStep; xong: () => void; san_choi?: boolean } = $props();

  const khe = $derived(buoc.code as unknown as Record<string, string | undefined>);
  let ma = $state('');
  let ket_qua = $state<KetQuaChay | null>(null);
  let dang_chay = $state(false);
  let so_lan_sai = $state(0);

  /** Cấu hình sân khấu, nếu bước này có thế giới. */
  const the_gioi = $derived(
    (buoc as { liveView?: { world?: { family?: string; params?: unknown } } }).liveView?.world ?? null,
  );
  const luoi = $derived(the_gioi?.family === 'grid-bot' ? (the_gioi.params as never) : null);
  const thanh_so = $derived(the_gioi?.family === 'number-line' ? (the_gioi.params as never) : null);
  const co_the_gioi = $derived(luoi ?? thanh_so);

  // Chỉ nạp mã khởi đầu MỘT lần cho mỗi bước. Không có bảo vệ này thì mỗi lần
  // Svelte tính lại `khe`, bài sẽ xoá sạch thứ người học đang gõ dở.
  let da_nap = $state('');
  $effect(() => {
    if (da_nap !== buoc.id) {
      ma = khe['starter'] ?? khe['readonly'] ?? '';
      ket_qua = null;
      so_lan_sai = 0;
      da_nap = buoc.id;
    }
  });

  async function chay() {
    dang_chay = true;
    ket_qua = null;
    try {
      // Mã kiểm tra chạy CÙNG không gian tên, ngay sau mã người học — đó là
      // điều làm `assert tinh_tien(3) == 9` kiểm được đúng hàm họ vừa viết.
      ket_qua = await chay_theo_ngon_ngu(
        khe['language'] ?? 'python',
        ma,
        san_choi ? undefined : khe['test'],
        co_the_gioi ?? undefined,
      );
      if (!ket_qua.ok && !san_choi) so_lan_sai += 1;
    } finally {
      dang_chay = false;
    }
  }

  // Sân chơi không có đáp án đúng, nên nút đi tiếp mở ngay từ đầu: bắt người
  // học "thắng" một sân chơi là biến nó thành bài tập, mà bài tập thì đã có ở
  // bước trước rồi.
  const dat = $derived(san_choi || ket_qua?.ok === true);

  let o_soan = $state<HTMLTextAreaElement | null>(null);

  /** Xử lý phím thụt lề.
   *
   *  Python quyết định khối lệnh bằng thụt lề, nên mặc định của trình duyệt
   *  (Tab chuyển focus) làm ô soạn không dùng được: người học viết `if`, bấm
   *  Tab, con trỏ nhảy sang nút bấm — và họ tưởng mình vừa làm sai Python.
   */
  function phim(e: KeyboardEvent) {
    const el = e.currentTarget as HTMLTextAreaElement;
    const truoc = { van: el.value, dau: el.selectionStart, cuoi: el.selectionEnd };
    let sau = null;

    if (e.key === 'Tab') sau = e.shiftKey ? tab_nguoc(truoc) : tab(truoc);
    else if (e.key === 'Enter') sau = enter(truoc);
    else if (e.key === 'Backspace') sau = backspace(truoc);
    if (!sau) return;

    e.preventDefault();
    ma = sau.van;
    // Đặt con trỏ SAU khi Svelte ghi giá trị mới, nếu không nó nhảy về cuối.
    queueMicrotask(() => el.setSelectionRange(sau.dau, sau.cuoi));
  }

  function dat_lai() {
    ma = khe['starter'] ?? '';
    ket_qua = null;
    o_soan?.focus();
  }
</script>

<div class="bai-code">
  <RichText noi_dung={buoc.body} />

  {#if luoi}
    <SanKhau {luoi} su_kien={(ket_qua?.suKien ?? []) as never} thang={ket_qua?.thang ?? false} />
  {:else if thanh_so}
    <ThanhSo thanh={thanh_so} su_kien={(ket_qua?.suKien ?? []) as never} thang={ket_qua?.thang ?? false} />
  {/if}

  <label class="soan">
    <span class="nhan">{san_choi ? 'Sân chơi — đổi gì cũng được' : 'Mã của bạn'}</span>
    <textarea
      bind:this={o_soan}
      bind:value={ma}
      onkeydown={phim}
      spellcheck="false"
      autocapitalize="off"
      rows={Math.max(4, ma.split('\n').length + 1)}
    ></textarea>
  </label>

  <div class="hang">
    <button class="chay" onclick={chay} disabled={dang_chay}>
      {dang_chay ? 'Đang chạy…' : 'Chạy thử'}
    </button>
    {#if dat}<button class="tiep" onclick={xong}>{san_choi ? 'Xong, đi tiếp' : 'Đi tiếp'}</button>{/if}
    <!-- Đặt lại phải luôn có: người học thử một hướng, đi lạc, rồi mắc kẹt vì
         không còn biết mã ban đầu ra sao. Bắt họ nhớ là bắt sai người. -->
    {#if ma !== (khe['starter'] ?? '')}
      <button class="dat-lai" onclick={dat_lai}>Đặt lại</button>
    {/if}
  </div>

  {#if ket_qua}
    <div class="ket-qua" class:dat class:truot={!dat}>
      {#if ket_qua.xuat}
        <span class="nhan">Máy in ra</span>
        <pre>{ket_qua.xuat}</pre>
      {/if}

      {#each ket_qua.chanDoan as c (c.ma + c.dong)}
        <div class="chan-doan">
          <p class="thong-diep"><code>{c.ma}</code> {c.thongDiep}</p>
          {#if c.viSao}<p class="vi-sao">{c.viSao}</p>{/if}
          {#each c.cachSua ?? [] as s}<p class="cach-sua">→ {s}</p>{/each}
        </div>
      {/each}

      {#if dat && !san_choi}
        <Byte tam_trang="reo-len" co={48} loi_thoai="Chạy đúng rồi." />
      {/if}
    </div>
  {/if}

  {#if !san_choi && buoc.hints && (so_lan_sai > 0 || ket_qua)}
    <GoiY thang={buoc.hints} />
  {/if}
</div>

<style>
  .soan { display: block; margin-top: 1.2rem; }
  .nhan {
    display: block; font-size: 0.76rem; letter-spacing: 0.06em;
    text-transform: uppercase; color: var(--chu-mo); margin-bottom: 0.35rem;
  }
  textarea {
    width: 100%; background: var(--nen-ma); color: var(--chu-nhat);
    border: 1.5px solid var(--vien); border-radius: 10px;
    padding: 0.9rem 1rem; font-family: var(--font-ma); font-size: 0.94rem;
    line-height: 1.65; resize: vertical; tab-size: 4;
  }
  textarea:focus { outline: none; border-color: var(--nhan); }
  .hang { display: flex; gap: 0.7rem; margin-top: 0.9rem; }
  .chay {
    background: var(--nhan); color: var(--nen); border: none; border-radius: 999px;
    padding: 0.55rem 1.3rem; font: inherit; font-weight: 600; cursor: pointer;
  }
  .chay:disabled { opacity: 0.55; cursor: default; }
  .tiep {
    background: none; border: 1.5px solid var(--dung); color: var(--dung);
    border-radius: 999px; padding: 0.55rem 1.3rem; font: inherit; font-weight: 600; cursor: pointer;
  }
  .dat-lai {
    background: none; border: none; color: var(--chu-mo); font: inherit;
    font-size: 0.86rem; cursor: pointer; padding: 0.55rem 0.4rem;
  }
  .dat-lai:hover { color: var(--nhan); }
  .ket-qua {
    margin-top: 1.1rem; padding: 1rem 1.1rem; border-radius: 12px;
    background: var(--nen-o); border-left: 3px solid var(--vien);
    display: flex; flex-direction: column; gap: 0.7rem;
  }
  .ket-qua.dat { border-left-color: var(--dung); background: var(--nen-dung); }
  .ket-qua.truot { border-left-color: var(--luu-y); background: var(--nen-luu-y); }
  pre {
    margin: 0; font-family: var(--font-ma); font-size: 0.92rem;
    white-space: pre-wrap; word-break: break-word;
  }
  .chan-doan p { margin: 0 0 0.35rem; line-height: 1.65; }
  .thong-diep { font-weight: 600; }
  .vi-sao { color: var(--chu-mo); }
  .cach-sua { color: var(--nhan); }
</style>
