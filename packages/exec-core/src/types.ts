/**
 * Hợp đồng chung cho mọi bộ thực thi code của người học.
 *
 * Ba ngôn ngữ chạy bằng ba công nghệ hoàn toàn khác nhau — Rust qua interpreter
 * WASM tự viết, Python qua Pyodide (CPython thật), TypeScript qua Sucrase —
 * nhưng phần còn lại của app chỉ được thấy đúng một giao diện này.
 *
 * Lý do: bài học, hệ chấm điểm và giao diện đều không nên biết ngôn ngữ nào
 * chạy bằng gì. Khi ta đổi cách chạy Rust (xem ADR-001), không dòng nào ở tầng
 * trên phải sửa.
 */

export type NgonNgu = 'rust' | 'python' | 'typescript';

/** Mức nghiêm trọng của một chẩn đoán. */
export type MucChanDoan = 'loi' | 'canh_bao';

/**
 * Một chẩn đoán, viết cho người mới học.
 *
 * Hình dạng này khớp đúng JSON mà `byte-rust` trả về, và hai engine còn lại
 * phải chuyển lỗi của chúng về cùng dạng — nếu không, giao diện sẽ phải rẽ
 * nhánh theo ngôn ngữ, và chất lượng thông báo sẽ chênh lệch giữa các bài học.
 */
export interface ChanDoan {
  /** Mã ổn định, ví dụ `BR0530`. Bài học dùng nó để mở đúng trang giải thích. */
  ma: string;
  muc: MucChanDoan;
  /** Tầng 1 — chuyện gì xảy ra, một câu, không thuật ngữ. */
  thongDiep: string;
  /** Dòng, đếm từ 1. */
  dong: number;
  /** Cột, đếm từ 1, tính theo KÝ TỰ (không phải byte). */
  cot: number;
  /** Số ký tự cần tô sáng. */
  doDai: number;
  /** Tầng 2 — vì sao ngôn ngữ không cho phép. */
  viSao: string | null;
  /** Tầng 3 — các cách sửa cụ thể. */
  cachSua: string[];
  /** Khái niệm liên quan, để mở bài học tương ứng. */
  khaiNiem: string | null;
  /** Bản kết xuất đầy đủ có trích mã nguồn và con trỏ chỉ lỗi. */
  vanBan: string;
}

export interface KetQuaChay {
  /** Không có chẩn đoán mức `loi`. */
  ok: boolean;
  /** Những gì chương trình in ra. */
  xuat: string;
  chanDoan: ChanDoan[];
  /** Thời gian chạy, mili-giây. */
  thoiGianMs: number;
  /** Bị dừng vì vượt ngân sách thời gian/bước. */
  biNgat: boolean;
}

export interface TuyChonChay {
  /**
   * Ngân sách thời gian. Hết hạn thì engine phải dừng và trả kết quả,
   * KHÔNG được treo giao diện.
   */
  hetHanMs?: number;
  /** Chạy thêm đoạn kiểm tra sau mã của người học (dùng cho chấm bài). */
  maKiemTra?: string;
}

/**
 * Bộ thực thi cho một ngôn ngữ.
 *
 * Mọi cài đặt đều phải bảo đảm ba điều, không có ngoại lệ:
 *
 * 1. **Không bao giờ treo.** Vòng lặp vô hạn phải trả về `biNgat: true` trong
 *    thời gian hữu hạn. Đây là lỗi dễ gặp nhất của người mới học.
 * 2. **Không bao giờ báo pass sai.** Thà trả lỗi "chưa hỗ trợ" còn hơn nói
 *    thành công cho mã mà ta không thực sự hiểu.
 * 3. **Không chạy trên luồng giao diện.** Mã người học chạy trong Worker.
 */
export interface BoThucThi {
  readonly ngonNgu: NgonNgu;
  /** Nạp tài nguyên (WASM, runtime). Gọi nhiều lần vẫn an toàn. */
  sanSang(): Promise<void>;
  chay(ma: string, tuyChon?: TuyChonChay): Promise<KetQuaChay>;
  /** Giải phóng tài nguyên. */
  dong(): void;
}

/** Kết quả JSON thô từ `byte-rust`, trước khi đổi sang camelCase. */
export interface KetQuaThoRust {
  ok: boolean;
  xuat: string;
  chan_doan: Array<{
    ma: string;
    muc: MucChanDoan;
    thong_diep: string;
    dong: number;
    cot: number;
    do_dai: number;
    vi_sao: string | null;
    cach_sua: string[];
    khai_niem: string | null;
    van_ban: string;
  }>;
}

export function doiChanDoanRust(tho: KetQuaThoRust['chan_doan'][number]): ChanDoan {
  return {
    ma: tho.ma,
    muc: tho.muc,
    thongDiep: tho.thong_diep,
    dong: tho.dong,
    cot: tho.cot,
    doDai: tho.do_dai,
    viSao: tho.vi_sao,
    cachSua: tho.cach_sua,
    khaiNiem: tho.khai_niem,
    vanBan: tho.van_ban,
  };
}
