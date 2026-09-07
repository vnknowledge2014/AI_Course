<script lang="ts">
  import RichText from './RichText.svelte';
  import Byte from './Byte.svelte';
  import GoiY from './GoiY.svelte';
  import DuDoan from './DuDoan.svelte';
  import BaiCode from './BaiCode.svelte';
  import type { Step, CodeSlot, ByteMood } from '@byte/content-schema';

  let { buoc, xong }: { buoc: Step; xong: () => void } = $props();

  /** Ánh xạ tâm trạng của schema sang khuôn mặt vẽ được.
   *
   *  Cả hai bảng đều không có trạng thái tiêu cực. `blocked` và `dizzy` là
   *  Byte đang bí, không phải Byte đang chê người học — nên chúng ánh xạ sang
   *  khuôn mặt tò mò và nghỉ ngơi, không phải mặt buồn.
   */
  const KHUON: Record<ByteMood, 'binh-tinh' | 'to-mo' | 'vui' | 'nghi-ngoi' | 'reo-len'> = {
    idle: 'binh-tinh',
    thinking: 'nghi-ngoi',
    happy: 'vui',
    blocked: 'to-mo',
    dizzy: 'nghi-ngoi',
    curious: 'to-mo',
  };

  /** Đoạn mã để đọc: bản `starter` nếu có, không thì bản chỉ-đọc hoặc ví dụ.
   *  Bản `solution` KHÔNG BAO GIỜ hiện ở đây — nó chỉ nằm sau nấc cuối của
   *  thang gợi ý, và hiện sớm là làm hộ chứ không phải dạy. */
  function ma_hien(c: CodeSlot | undefined): { ma: string } | null {
    if (!c) return null;
    const v = c as unknown as Record<string, string | undefined>;
    const ma = v['starter'] ?? v['readonly'] ?? v['example'];
    return ma ? { ma } : null;
  }

  /** Lời thoại `enter` — thoại dẫn vào bước. Các trigger khác (`success`,
   *  `idle`) cần trạng thái lúc chạy nên chưa phát ở tầng này. */
  const thoai_vao = $derived((buoc.byte ?? []).filter((b) => b.trigger.on === 'enter'));
</script>

<section class="buoc">
  {#each thoai_vao as b (b.line)}
    <div class="thoai-byte">
      <Byte tam_trang={KHUON[b.mood]} co={56} />
      <div class="bong-bong"><RichText noi_dung={b.line} /></div>
    </div>
  {/each}

  {#if buoc.kind === 'predict'}
    <DuDoan buoc={buoc as never} {xong} />

  {:else if buoc.kind === 'code' || buoc.kind === 'sandbox' || buoc.kind === 'repair' || buoc.kind === 'refactor' || buoc.kind === 'assemble'}
    <!-- Sân chơi dùng chung thành phần với bài tập: cùng ô soạn, cùng nút
         chạy, cùng sân khấu. Khác đúng một điều — nó không chấm.
         repair/refactor/assemble biên dịch ra ĐÚNG hình dạng CodeStep
         (starter/solution/test/hints/validate) — xem content-compiler
         lesson.ts, case 'code'|'repair'|'refactor'|'assemble' dùng
         chung một nhánh — nên dùng lại BaiCode nguyên vẹn, không viết
         thành phần riêng. -->
    <BaiCode buoc={buoc as never} {xong} san_choi={buoc.kind === 'sandbox'} />

  {:else if buoc.kind === 'reflect'}
    <div class="ngam">
      <Byte tam_trang="nghi-ngoi" co={52} />
      <div><RichText noi_dung={buoc.prompt} /></div>
    </div>

  {:else}
    {#if 'body' in buoc && buoc.body}
      <RichText noi_dung={buoc.body as never} />
    {/if}

    {#if 'code' in buoc}
      {@const c = ma_hien(buoc.code as CodeSlot)}
      {#if c}
        <pre class="ma"><code>{c.ma}</code></pre>
      {/if}
    {/if}

    {#if 'hints' in buoc && buoc.hints}
      <GoiY thang={buoc.hints as never} />
    {/if}
  {/if}
</section>

<style>
  .buoc { display: flex; flex-direction: column; }
  .thoai-byte { display: flex; gap: 0.9rem; align-items: flex-start; margin-bottom: 1.3rem; }
  .bong-bong {
    background: var(--nen-o);
    border-radius: 4px 14px 14px 14px;
    padding: 0.8rem 1.05rem;
    flex: 1;
  }
  .ngam { display: flex; gap: 1rem; align-items: flex-start; }
  .ma {
    background: var(--nen-ma);
    padding: 1rem 1.1rem;
    border-radius: 10px;
    overflow-x: auto;
    font-family: var(--font-ma);
    font-size: 0.92rem;
    line-height: 1.6;
    margin: 1rem 0 0;
  }
</style>
