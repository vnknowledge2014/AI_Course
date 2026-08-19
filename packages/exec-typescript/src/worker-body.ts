/**
 * Phần chạy BÊN TRONG worker cho TypeScript.
 *
 * Tách riêng khỏi cách tạo worker để test được, và để cùng một đoạn mã này
 * dùng được cho Web Worker (trình duyệt, WebView) lẫn `worker_threads` (Node).
 */

import type { PhanHoiChay, YeuCauChay } from '@byte/exec-core';

/** Hàm chuyển TypeScript thành JavaScript. Tiêm vào để gói này không phụ thuộc bộ transpiler nào. */
export type BoChuyenDoi = (ma: string) => string;

/**
 * Chạy mã người học, thu lại mọi thứ nó in ra.
 *
 * Ghi đè `console` thay vì để nguyên, vì output phải hiện trong khung kết quả
 * của bài học chứ không phải trong DevTools mà người mới học không biết mở.
 */
export function chayTrongWorker(
  yeuCau: YeuCauChay,
  chuyenDoi: BoChuyenDoi,
): PhanHoiChay {
  const dong: string[] = [];
  const consoleGoc = { ...globalThis.console };

  const ghi = (...doiSo: unknown[]) => {
    dong.push(
      doiSo
        .map((x) => {
          if (typeof x === 'string') return x;
          try {
            return JSON.stringify(x);
          } catch {
            return String(x);
          }
        })
        .join(' '),
    );
  };

  globalThis.console = {
    ...consoleGoc,
    log: ghi,
    info: ghi,
    warn: ghi,
    error: ghi,
    debug: ghi,
  };

  try {
    const nguon = yeuCau.maKiemTra ? `${yeuCau.ma}\n${yeuCau.maKiemTra}` : yeuCau.ma;
    const js = chuyenDoi(nguon);
    // Bọc trong hàm để `return` ở cấp cao nhất không gây lỗi cú pháp.
    const f = new Function(`"use strict";\n${js}`);
    f();
    return { loai: 'xong', id: yeuCau.id, ok: true, xuat: dong.join('\n'), loi: null };
  } catch (e) {
    const loi = e instanceof Error ? `${e.name}: ${e.message}` : String(e);
    return { loai: 'xong', id: yeuCau.id, ok: false, xuat: dong.join('\n'), loi };
  } finally {
    globalThis.console = consoleGoc;
  }
}
