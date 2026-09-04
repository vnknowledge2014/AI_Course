/**
 * Phần chạy BÊN TRONG worker cho Python.
 *
 * Tách khỏi cách tạo worker để test được, và để cùng một đoạn mã này dùng được
 * cho Web Worker (trình duyệt, WebView của Tauri) lẫn `worker_threads` (Node,
 * khi chạy test).
 *
 * Đây là CPython thật biên dịch sang WASM, không phải một bản mô phỏng. Với
 * Python ta cố tình không tự viết interpreter: người học phải gặp đúng thông
 * báo lỗi, đúng hành vi số học, đúng thứ tự dict mà họ sẽ gặp khi rời khỏi
 * ứng dụng này. Một bản gần-đúng sẽ dạy sai ở đúng những chỗ khó thấy nhất.
 */

import type { PhanHoiChay, YeuCauChay } from '@byte/exec-core';
import { chayTrenLuoi, type CauHinhLuoi } from './the-gioi.js';
import { chayTrenThanhSo, type CauHinhThanhSo } from './the-gioi-so.js';

/** Bề mặt Pyodide mà worker này dùng tới — khai báo hẹp để không kéo cả kiểu của gói vào. */
export interface Pyodide {
  runPython(ma: string): unknown;
  setStdout(o: { batched: (s: string) => void }): void;
  setStderr(o: { batched: (s: string) => void }): void;
  globals: { get(ten: string): unknown; set(ten: string, v: unknown): void };
  /** Nạp một gói Pyodide (vd 'numpy') — dùng để nạp numpy MỘT LẦN lúc khởi
   * động worker, phục vụ nội dung Realm 8 (AI/numpy). */
  loadPackage(tenGoiHoacUrl: string): Promise<unknown>;
}

/**
 * Chạy mã người học và thu lại mọi thứ nó in ra.
 *
 * `stdout` và `stderr` đều được gom về một dòng chảy: người mới học không phân
 * biệt hai luồng đó, và trong khung kết quả của bài học thì thứ tự thời gian
 * mới là thứ đáng giữ.
 */
export function chayTrongWorkerPython(yeuCau: YeuCauChay, py: Pyodide): PhanHoiChay {
  const dong: string[] = [];
  const gom = { batched: (s: string) => dong.push(s) };
  py.setStdout(gom);
  py.setStderr(gom);

  try {
    // Bài có sân khấu: chạy trong thế giới lưới và trả kèm chuỗi sự kiện.
    if (yeuCau.luoi) {
      // Phân biệt thế giới bằng HÌNH DẠNG cấu hình, không bằng một trường
      // `family` riêng: cấu hình đã nói ra nó là thế giới nào (`rong`/`cao`
      // cho lưới, `tu`/`den` cho thanh số), và thêm một trường nữa chỉ tạo
      // thêm một chỗ để hai bên lệch nhau.
      const ch = yeuCau.luoi as Record<string, unknown>;
      const kq =
        typeof ch['tu'] === 'number' && typeof ch['den'] === 'number'
          ? chayTrenThanhSo(py, yeuCau.ma, ch as unknown as CauHinhThanhSo)
          : chayTrenLuoi(py, yeuCau.ma, ch as unknown as CauHinhLuoi);
      return {
        loai: 'xong',
        id: yeuCau.id,
        ok: kq.thang,
        xuat: dong.join('\n'),
        loi: kq.thang ? null : kq.vi_sao,
        suKien: kq.su_kien,
        thang: kq.thang,
        viSao: kq.vi_sao,
      };
    }
    // Mã kiểm tra nối SAU mã người học, cùng một không gian tên — đó là điều
    // làm `assert tinh_tien(3) == 9` kiểm được đúng hàm họ vừa viết.
    const nguon = yeuCau.maKiemTra ? `${yeuCau.ma}\n${yeuCau.maKiemTra}` : yeuCau.ma;
    py.runPython(nguon);
    return { loai: 'xong', id: yeuCau.id, ok: true, xuat: dong.join('\n'), loi: null };
  } catch (e) {
    // Pyodide ném lỗi JS mang nguyên traceback Python trong `message`. Giữ
    // nguyên văn: `index.ts` mới là chỗ cắt bớt khung stack của chính Pyodide
    // và dịch thành chẩn đoán tiếng Việt.
    const loi = e instanceof Error ? e.message : String(e);
    return { loai: 'xong', id: yeuCau.id, ok: false, xuat: dong.join('\n'), loi };
  }
}

/**
 * Gắn worker vào một Pyodide đã nạp xong.
 *
 * Nhận `pyodide` như một tham số thay vì tự nạp: nơi lấy file runtime khác
 * nhau giữa web (`/pyodide/`) và bản đóng gói Tauri (đường dẫn tương đối), và
 * gói này không được biết mình đang chạy ở đâu.
 */
export function gan(
  py: Pyodide,
  guiDi: (tin: PhanHoiChay | { loai: 'san_sang' }) => void,
  nghe: (xuLy: (yeuCau: YeuCauChay) => void) => void,
): void {
  nghe((yeuCau) => guiDi(chayTrongWorkerPython(yeuCau, py)));
  guiDi({ loai: 'san_sang' });
}
