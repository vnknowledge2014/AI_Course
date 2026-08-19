/**
 * Giao thức giữa host và worker.
 *
 * Cả Python lẫn TypeScript đều chạy trong Worker vì cùng một lý do: JavaScript
 * không có cách nào ngắt một vòng lặp đang chạy từ bên trong. Cách duy nhất
 * chắc chắn hiệu quả là **giết cả worker từ bên ngoài** — nên mã của người học
 * bắt buộc phải sống ở một luồng khác luồng giao diện.
 *
 * (Rust không cần cơ chế này vì interpreter WASM tự có ngân sách nhiên liệu,
 *  nhưng vẫn dùng chung đường đi để hành vi nhất quán.)
 */

export interface YeuCauChay {
  loai: 'chay';
  /** Định danh để ghép yêu cầu với phản hồi. */
  id: number;
  ma: string;
  maKiemTra?: string;
}

export interface PhanHoiChay {
  loai: 'xong';
  id: number;
  ok: boolean;
  xuat: string;
  /** Thông báo lỗi thô của runtime, host sẽ chuyển thành chẩn đoán. */
  loi: string | null;
}

export interface PhanHoiSanSang {
  loai: 'san_sang';
}

export type TinNhanTuWorker = PhanHoiChay | PhanHoiSanSang;
