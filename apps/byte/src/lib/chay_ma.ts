/** Cầu nối giữa giao diện và bộ thực thi.
 *
 *  Một worker DÙNG CHUNG cho cả phiên: Pyodide mất vài giây để nạp, tạo lại
 *  cho mỗi lần bấm chạy thì mỗi lần bấm là một lần chờ. Đổi lại, khi worker bị
 *  giết vì hết giờ thì lần chạy kế tiếp phải nạp lại — `BoThucThiPython` tự lo
 *  việc đó.
 */
import { BoThucThiPython } from '@byte/exec-python';
import { BoThucThiRust, ketQuaTuThoRust } from '@byte/exec-rust';
import type { CongWorker, KetQuaChay, KetQuaThoRust, TinNhanTuWorker } from '@byte/exec-core';

function tao_cong(): CongWorker {
  const w = new Worker(new URL('./python.worker.ts', import.meta.url), { type: 'module' });
  return {
    gui: (tin) => w.postMessage(tin),
    khiNhan: (xu_ly: (t: TinNhanTuWorker) => void) => {
      w.onmessage = (e: MessageEvent<TinNhanTuWorker>) => xu_ly(e.data);
    },
    giet: () => w.terminate(),
  };
}

let may: BoThucThiPython | null = null;

export function bo_thuc_thi(): BoThucThiPython {
  may ??= new BoThucThiPython(tao_cong);
  return may;
}

export async function chay_python(
  ma: string,
  ma_kiem_tra?: string,
  luoi?: unknown,
): Promise<KetQuaChay> {
  const m = bo_thuc_thi();
  await m.sanSang();
  // Bài có sân khấu chấm bằng LUẬT CHƠI (nhặt hết viên, không đâm tường), nên
  // nó không chạy khối `test` — hai cách chấm chồng lên nhau chỉ làm người học
  // trượt vì một lý do mà màn hình không hề nói tới.
  if (luoi) return m.chay(ma, { luoi });
  return ma_kiem_tra ? m.chay(ma, { maKiemTra: ma_kiem_tra }) : m.chay(ma);
}

/* ── Rust ───────────────────────────────────────────────────────────────── */

/** Bộ thực thi Rust — có HAI đường chạy kết thúc ở cùng một chỗ.
 *
 *  Trong vỏ Tauri (desktop/mobile) thì gọi command native `invoke('chay_rust')`
 *  — nhanh hơn và khỏi nạp 476 KB WASM. Ngoài trình duyệt thuần thì nạp
 *  `byte_rust.wasm`. Cả hai đều nhận về ĐÚNG cùng một chuỗi JSON từ
 *  `byte_rust::wasm::chay_thanh_json` và cùng đi qua hàm diễn dịch
 *  `ketQuaTuThoRust`, nên kết quả chấm khớp hệt nhau theo CẤU TRÚC chứ không
 *  phải theo hy vọng: một bài không thể đạt trên Mac mà trượt trên Android.
 *
 *  Đường WASM không cần Worker: module tự có ngân sách nhiên liệu và giới hạn
 *  độ sâu, nên vòng lặp vô hạn của người học bị chặn ở tầng Rust chứ không cần
 *  host giết ai cả. Đó là khác biệt thật giữa một interpreter mình viết và một
 *  runtime mượn — và là lý do ADR-001 chọn viết interpreter.
 *
 *  Nhận biết vỏ Tauri: Tauri 2 tiêm `window.__TAURI_INTERNALS__` vào mọi
 *  WebView nó quản lý (`window.__TAURI__` chỉ có khi bật `withGlobalTauri`, mà
 *  `tauri.conf.json` không bật). Nếu `invoke` ném — ví dụ vỏ mới thiếu command
 *  — quay về đường WASM: cùng một crate, cùng một JSON, nên chấm vẫn đúng,
 *  chỉ chậm hơn; lỗi được báo qua `console.warn` thay vì nuốt lặng.
 */
let may_rust: BoThucThiRust | null = null;

export function bo_thuc_thi_rust(): BoThucThiRust {
  may_rust ??= new BoThucThiRust(async () => {
    const r = await fetch(`${import.meta.env.BASE_URL}wasm/byte_rust.wasm`);
    if (!r.ok) throw new Error(`không nạp được byte_rust.wasm (HTTP ${r.status})`);
    return r.arrayBuffer();
  });
  return may_rust;
}

function trong_tauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

async function chay_rust_native(ma: string, ma_kiem_tra?: string): Promise<KetQuaChay> {
  // Import ĐỘNG: @tauri-apps/api chỉ có ý nghĩa trong vỏ Tauri — import tĩnh sẽ
  // kéo nó vào bundle web thuần dù không bao giờ gọi tới.
  const { invoke } = await import('@tauri-apps/api/core');
  // Command nhận nguyên chương trình (gồm cả phần kiểm tra), y hệt cách
  // BoThucThiRust.chay ghép `ma\nmaKiemTra` trước khi đưa xuống WASM.
  const nguon = ma_kiem_tra ? `${ma}\n${ma_kiem_tra}` : ma;
  const t0 = performance.now();
  const json = await invoke<string>('chay_rust', { nguon });
  return ketQuaTuThoRust(JSON.parse(json) as KetQuaThoRust, performance.now() - t0);
}

export async function chay_rust(ma: string, ma_kiem_tra?: string): Promise<KetQuaChay> {
  if (trong_tauri()) {
    try {
      return await chay_rust_native(ma, ma_kiem_tra);
    } catch (loi) {
      console.warn('chay_rust native thất bại, quay về đường WASM:', loi);
    }
  }
  const m = bo_thuc_thi_rust();
  return ma_kiem_tra ? m.chay(ma, { maKiemTra: ma_kiem_tra }) : m.chay(ma);
}

/** Chạy mã theo ĐÚNG ngôn ngữ của bước, không đoán.
 *
 *  Đoán ngôn ngữ từ nội dung mã là chỗ hỏng âm thầm chờ sẵn: một bài Rust có
 *  dòng `let x = 1;` trông không khác gì TypeScript, và chạy nhầm engine sẽ
 *  cho ra một chẩn đoán vô nghĩa mà người học không có cách nào lần ra.
 */
export async function chay(
  ngon_ngu: string,
  ma: string,
  ma_kiem_tra?: string,
  the_gioi?: unknown,
): Promise<KetQuaChay> {
  if (ngon_ngu === 'rust') return chay_rust(ma, ma_kiem_tra);
  if (ngon_ngu === 'typescript') return chay_typescript(ma, ma_kiem_tra);
  return chay_python(ma, ma_kiem_tra, the_gioi);
}

/* ── TypeScript ─────────────────────────────────────────────────────────── */

/** Bộ thực thi TypeScript, nạp LƯỜI.
 *
 *  Trình biên dịch `typescript` nặng 8,7 MB và bảng `lib.d.ts` thêm 3,1 MB.
 *  Nạp chúng lúc mở ứng dụng nghĩa là bắt mọi người học Python trả giá cho
 *  một thứ họ không dùng — nên chúng chỉ được nạp khi có bài TypeScript thật
 *  sự chạy, và Vite tách chúng thành chunk riêng.
 *
 *  Kiểm kiểu ở HOST chứ không trong worker: mã sai kiểu không được xuống tới
 *  worker, đúng luật chung của cả ba engine (ADR-002).
 */
let may_ts: import('@byte/exec-typescript').BoThucThiTypeScript | null = null;

async function tao_may_ts() {
  if (may_ts) return may_ts;

  const [{ BoThucThiTypeScript, kiemKieu }, TS, lib] = await Promise.all([
    import('@byte/exec-typescript'),
    import('typescript'),
    fetch(`${import.meta.env.BASE_URL}ts-lib/lib.json`).then((r) => {
      if (!r.ok) throw new Error(`không nạp được lib.d.ts (HTTP ${r.status})`);
      return r.json() as Promise<Record<string, string>>;
    }),
  ]);

  may_ts = new BoThucThiTypeScript(
    () => {
      const w = new Worker(new URL('./typescript.worker.ts', import.meta.url), { type: 'module' });
      return {
        gui: (tin) => w.postMessage(tin),
        khiNhan: (xu_ly: (t: TinNhanTuWorker) => void) => {
          w.onmessage = (e: MessageEvent<TinNhanTuWorker>) => xu_ly(e.data);
        },
        giet: () => w.terminate(),
      };
    },
    (ma) => kiemKieu(TS as never, ma, (ten) => lib[ten]),
  );
  return may_ts;
}

export async function chay_typescript(ma: string, ma_kiem_tra?: string): Promise<KetQuaChay> {
  const m = await tao_may_ts();
  return ma_kiem_tra ? m.chay(ma, { maKiemTra: ma_kiem_tra }) : m.chay(ma);
}
