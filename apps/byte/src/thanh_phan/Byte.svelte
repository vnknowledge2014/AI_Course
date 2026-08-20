<script lang="ts">
  /** Byte — nhân vật dẫn đường.
   *
   *  Bảng tâm trạng CỐ TÌNH không có trạng thái tiêu cực. Người mới học đã đủ
   *  thấy mình kém; một nhân vật hướng dẫn tỏ ra thất vọng sẽ biến sai lầm
   *  thành nỗi xấu hổ, mà xấu hổ thì làm người ta bỏ học chứ không làm họ giỏi
   *  lên. Xem MASTERPLAN §0 quyết định 11.
   */
  type TamTrang = 'binh-tinh' | 'to-mo' | 'vui' | 'nghi-ngoi' | 'reo-len';

  let {
    tam_trang = 'binh-tinh',
    loi_thoai = '',
    co = 72,
  }: { tam_trang?: TamTrang; loi_thoai?: string; co?: number } = $props();

  const mat: Record<TamTrang, { trai: string; phai: string; mieng: string }> = {
    'binh-tinh': { trai: 'M -7 -3 v 6', phai: 'M 7 -3 v 6', mieng: 'M -5 8 q 5 3 10 0' },
    'to-mo':     { trai: 'M -7 -4 v 7', phai: 'M 7 -2 v 5', mieng: 'M -4 8 q 4 4 8 -1' },
    'vui':       { trai: 'M -9 -1 q 2 -5 4 0', phai: 'M 5 -1 q 2 -5 4 0', mieng: 'M -6 6 q 6 6 12 0' },
    'nghi-ngoi': { trai: 'M -9 0 h 4', phai: 'M 5 0 h 4', mieng: 'M -4 9 h 8' },
    'reo-len':   { trai: 'M -9 -2 q 2 -5 4 0', phai: 'M 5 -2 q 2 -5 4 0', mieng: 'M -6 5 q 6 9 12 0' },
  };
  const m = $derived(mat[tam_trang]);
</script>

<div class="byte" style="--co: {co}px">
  <svg viewBox="-32 -32 64 64" width={co} height={co} aria-label="Byte" role="img">
    <rect x="-26" y="-24" width="52" height="48" rx="14" class="than" />
    <g class="net">
      <path d={m.trai} /><path d={m.phai} /><path d={m.mieng} />
    </g>
  </svg>
  {#if loi_thoai}
    <p class="thoai">{loi_thoai}</p>
  {/if}
</div>

<style>
  .byte { display: flex; align-items: center; gap: 0.9rem; }
  .than { fill: var(--byte-than); stroke: var(--byte-vien); stroke-width: 2.5; }
  .net {
    fill: none;
    stroke: var(--byte-vien);
    stroke-width: 3;
    stroke-linecap: round;
  }
  .thoai {
    margin: 0;
    font-size: 1rem;
    line-height: 1.6;
    color: var(--chu-nhat);
    max-width: 46ch;
  }
</style>
