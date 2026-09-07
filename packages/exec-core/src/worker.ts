/**
 * Trừu tượng hoá Worker và chẩn đoán quá giờ.
 *
 * Đặt ở `exec-core` chứ không ở một engine cụ thể: cả TypeScript lẫn Python đều
 * cần, và nếu để ở một bên thì bên kia phải phụ thuộc ngược vào engine anh em —
 * một quan hệ sai về mặt kiến trúc.
 */

import type { ChanDoan } from './types.js';
import type { TinNhanTuWorker, YeuCauChay } from './worker-protocol.js';

/**
 * Cổng tới một worker.
 *
 * Trừu tượng hoá vì cách tạo worker khác nhau ở mỗi nền tảng: `Worker` của
 * trình duyệt, `worker_threads` của Node, WebView của Tauri. Engine không nên
 * biết mình đang chạy ở đâu.
 */
export interface CongWorker {
  gui(tin: YeuCauChay): void;
  khiNhan(f: (tin: TinNhanTuWorker) => void): void;
  /** Chấm dứt worker. Đây là cách DUY NHẤT ngắt được vòng lặp vô hạn trong JS. */
  giet(): void;
}

/**
 * Gửi MỘT yêu cầu `chay` qua `CongWorker`, đua với đồng hồ hết giờ.
 *
 * Đây là lõi dùng chung của mọi engine chạy trong Worker (Python,
 * TypeScript — Rust không cần vì WASM tự có ngân sách nhiên liệu, xem
 * `packages/exec-rust`): gửi yêu cầu, đợi ĐÚNG phản hồi khớp `id` (không
 * phải phản hồi cũ còn sót lại từ lần gọi trước), và nếu hết giờ TRƯỚC
 * khi khớp được thì giết worker rồi trả `null` — cách duy nhất ngắt được
 * một vòng lặp vô hạn đang chạy trong JavaScript.
 *
 * Trả `null` khi hết giờ, hoặc phản hồi khớp `id` khi worker trả lời kịp.
 * Phần XỬ LÝ phản hồi đó (dịch traceback Python, dịch lỗi V8, v.v.) vẫn ở
 * lại từng engine — nó khác nhau theo ngôn ngữ, không phải phần dùng
 * chung.
 */
export function chayCoHetGio(
  cong: CongWorker,
  yeuCau: Omit<YeuCauChay, 'loai'>,
  hetHanMs: number,
  khiHetGio: () => void,
): Promise<TinNhanTuWorker | null> {
  return new Promise((resolve) => {
    const dongHo = setTimeout(() => {
      cong.giet();
      khiHetGio();
      resolve(null);
    }, hetHanMs);

    cong.khiNhan((tin) => {
      if (tin.loai === 'xong' && tin.id === yeuCau.id) {
        clearTimeout(dongHo);
        resolve(tin);
      }
    });

    cong.gui({ loai: 'chay', ...yeuCau });
  });
}

/** Chẩn đoán khi worker bị giết vì quá giờ. */
export function chanDoanQuaGio(hetHanMs: number): ChanDoan {
  const giay = hetHanMs / 1000;
  return {
    ma: 'EX0500',
    muc: 'loi',
    thongDiep: `chương trình chạy quá ${giay} giây nên đã bị dừng`,
    dong: 1,
    cot: 1,
    doDai: 1,
    viSao:
      'nhiều khả năng có một vòng lặp không bao giờ kết thúc — điều kiện dừng không bao giờ thành `false`',
    cachSua: [
      'kiểm tra xem biến trong điều kiện vòng lặp có thực sự thay đổi không',
      'với vòng lặp vô hạn có chủ đích, chắc chắn có một nhánh thoát chạy tới được',
    ],
    khaiNiem: 'vòng lặp',
    vanBan: `lỗi [EX0500]: chương trình chạy quá ${giay} giây nên đã bị dừng`,
  };
}
