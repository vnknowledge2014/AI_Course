<script lang="ts">
  import RichText from './RichText.svelte';
  import GoiY from './GoiY.svelte';
  import Byte from './Byte.svelte';
  import { chay_python } from '../lib/chay_ma';
  import type { CodeStep, KetQuaChay } from './kieu';

  let { buoc, xong }: { buoc: CodeStep; xong: () => void } = $props();

  const khe = $derived(buoc.code as unknown as Record<string, string | undefined>);
  let ma = $state('');
  let ket_qua = $state<KetQuaChay | null>(null);
  let dang_chay = $state(false);
  let so_lan_sai = $state(0);

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
      ket_qua = await chay_python(ma, khe['test']);
      if (!ket_qua.ok) so_lan_sai += 1;
    } finally {
      dang_chay = false;
    }
  }

  const dat = $derived(ket_qua?.ok === true);
</script>

<div class="bai-code">
  <RichText noi_dung={buoc.body} />

  <label class="soan">
    <span class="nhan">Mã của bạn</span>
    <textarea
      bind:value={ma}
      spellcheck="false"
      autocapitalize="off"
      rows={Math.max(4, ma.split('\n').length + 1)}
    ></textarea>
  </label>

  <div class="hang">
    <button class="chay" onclick={chay} disabled={dang_chay}>
      {dang_chay ? 'Đang chạy…' : 'Chạy thử'}
    </button>
    {#if dat}<button class="tiep" onclick={xong}>Đi tiếp</button>{/if}
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

      {#if dat}
        <Byte tam_trang="reo-len" co={48} loi_thoai="Chạy đúng rồi." />
      {/if}
    </div>
  {/if}

  {#if buoc.hints && (so_lan_sai > 0 || ket_qua)}
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
