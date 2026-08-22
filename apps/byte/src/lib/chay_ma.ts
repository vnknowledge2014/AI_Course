/** Cầu nối giữa giao diện và bộ thực thi.
 *
 *  Một worker DÙNG CHUNG cho cả phiên: Pyodide mất vài giây để nạp, tạo lại
 *  cho mỗi lần bấm chạy thì mỗi lần bấm là một lần chờ. Đổi lại, khi worker bị
 *  giết vì hết giờ thì lần chạy kế tiếp phải nạp lại — `BoThucThiPython` tự lo
 *  việc đó.
 */
import { BoThucThiPython } from '@byte/exec-python';
import { BoThucThiRust } from '@byte/exec-rust';
import type { CongWorker, KetQuaChay, TinNhanTuWorker } from '@byte/exec-core';

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

/** Bộ thực thi Rust — `byte_rust.wasm`, 476 KB.
 *
 *  Không cần Worker: module WASM tự có ngân sách nhiên liệu và giới hạn độ
 *  sâu, nên vòng lặp vô hạn của người học bị chặn ở tầng Rust chứ không cần
 *  host giết ai cả. Đó là khác biệt thật giữa một interpreter mình viết và
 *  một runtime mượn — và là lý do ADR-001 chọn viết interpreter.
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

export async function chay_rust(ma: string, ma_kiem_tra?: string): Promise<KetQuaChay> {
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
  return chay_python(ma, ma_kiem_tra, the_gioi);
}
