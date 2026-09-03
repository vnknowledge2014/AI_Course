/**
 * `SimDisk` — block device mô phỏng cho Realm 6 (Database).
 *
 * Vì sao không dùng `open()` thật (xem MASTERPLAN.md §4): Pyodide chạy trên
 * MEMFS trong RAM, `os.fsync()` ở đó là no-op, không có khái niệm sector, và
 * `terminate()` Worker xoá sạch MEMFS — nên sau một "crash" mô phỏng sẽ
 * không còn gì để phục hồi, cũng không có torn write hay latent sector error
 * để dạy. `SimDisk` dựng lại đúng những thứ một hệ điều hành thật có: một
 * platter bền (sống sót qua `crash()`) và một cache ghi CHƯA `fsync()` (biến
 * mất khi `crash()`).
 *
 * Đây là bản THAM CHIẾU (TypeScript) — bản Python song sinh ở `py/simdisk.py`
 * cùng ngữ nghĩa, dùng cho các bài Python (vd R6-1 q00 `[PY, SimDisk]`). Bài
 * học không `import` gói này lúc chạy (sandbox Pyodide/WASM không nạp được
 * gói workspace) — mỗi bài tự chép phần code cần dùng vào khối `readonly`,
 * đúng quy ước đã dùng xuyên suốt dự án (không import chéo giữa các bài).
 * Gói này là NGUỒN SỰ THẬT để đối chiếu ngữ nghĩa lúc viết bài, và có bộ test
 * riêng để tự nó luôn đúng.
 *
 * v1 (đủ cho R6-1 q00 "Chiếc hộp giày của Byte"): read/write/fsync/crash cơ
 * bản, không có fault injection. Torn write / lost fsync / latent sector
 * error / misdirected write / crash tất định thứ N (cho q02, q18) sẽ thêm
 * khi viết tới các quest đó — không xây trước khi có bài cần dùng.
 */

export const KICH_THUOC_SECTOR_MAC_DINH = 512;
export const SO_LUONG_SECTOR_MAC_DINH = 16;

export interface TuyChonSimDisk {
  /** Tổng số sector trên đĩa. */
  soLuongSector?: number;
  /** Kích thước MỖI sector, tính bằng byte — `write()` phải khớp đúng số này. */
  kichThuocSector?: number;
}

/**
 * Đĩa mô phỏng: `soLuongSector` sector, mỗi sector `kichThuocSector` byte.
 *
 * Mô hình hai tầng, đúng cách một đĩa thật hoạt động:
 * - `platter` — lưu trữ BỀN, chỉ cập nhật khi `fsync()`, sống sót qua `crash()`.
 * - `cache` — ghi CHƯA `fsync()`; `read()` vẫn thấy được (vì đọc lại ngay sau
 *   ghi luôn thấy dữ liệu mới trên một hệ điều hành thật), nhưng `crash()`
 *   xoá sạch nó — đây chính là rủi ro `fsync()` tồn tại để phòng.
 */
export class SimDisk {
  readonly soLuongSector: number;
  readonly kichThuocSector: number;

  private readonly platter = new Map<number, Uint8Array>();
  private readonly cache = new Map<number, Uint8Array>();

  constructor(tuyChon: TuyChonSimDisk = {}) {
    this.soLuongSector = tuyChon.soLuongSector ?? SO_LUONG_SECTOR_MAC_DINH;
    this.kichThuocSector = tuyChon.kichThuocSector ?? KICH_THUOC_SECTOR_MAC_DINH;
  }

  private kiemTraSector(sector: number): void {
    if (!Number.isInteger(sector) || sector < 0 || sector >= this.soLuongSector) {
      throw new RangeError(
        `sector ${sector} ngoài phạm vi [0, ${this.soLuongSector})`,
      );
    }
  }

  /** Đọc một sector. Sector chưa từng ghi trả về toàn số 0 (đúng như đĩa thật mới format). */
  read(sector: number): Uint8Array {
    this.kiemTraSector(sector);
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }

  /** Ghi một sector VÀO CACHE — chưa bền cho tới khi `fsync()`. */
  write(sector: number, data: Uint8Array): void {
    this.kiemTraSector(sector);
    if (data.length !== this.kichThuocSector) {
      throw new RangeError(
        `write() cần đúng ${this.kichThuocSector} byte, nhận ${data.length}`,
      );
    }
    this.cache.set(sector, data.slice());
  }

  /** Đẩy TOÀN BỘ cache xuống platter — sau lệnh này, mọi ghi đã bền. */
  fsync(): void {
    for (const [sector, data] of this.cache) this.platter.set(sector, data);
    this.cache.clear();
  }

  /** Mô phỏng mất điện: ghi CHƯA fsync biến mất, platter giữ nguyên. */
  crash(): void {
    this.cache.clear();
  }

  /** Còn ghi nào chưa fsync không — dùng để bài học kiểm tra tình huống trước khi `crash()`. */
  coGhiChuaFsync(): boolean {
    return this.cache.size > 0;
  }
}
