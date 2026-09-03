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
 * v1 (R6-1 q00): read/write/fsync/crash cơ bản.
 *
 * v2 (R6-1 q02 "Khi điện mất"): thêm hai kiểu lỗi, CHỦ ĐỘNG kích hoạt (không
 * random — bài học cần tái lập được y hệt mỗi lần chạy):
 * - `boQuaFsyncKeTiep()` — "lost fsync": lần `fsync()` tiếp theo LÀ no-op
 *   hoàn toàn (không đẩy cache xuống platter, KHÔNG báo lỗi) — mô phỏng
 *   driver/OS "nói dối" đã flush. `read()` vẫn thấy đúng (cache còn nguyên,
 *   chưa bị xoá) — lời nói dối CHỈ lộ ra khi `crash()` xoá cache.
 * - `danhDauTornGhi(sector, soByteThanhCong)` — "torn write": lần `fsync()`
 *   tiếp theo GHI THÀNH CÔNG chỉ `soByteThanhCong` byte ĐẦU của sector đó
 *   xuống platter, phần CÒN LẠI giữ nguyên dữ liệu CŨ (hoặc số 0 nếu sector
 *   chưa từng ghi) — mô phỏng mất điện giữa chừng một lần ghi vật lý.
 *
 * latent sector error / misdirected write / crash tất định thứ N (cho q18)
 * CHƯA xây — thêm khi viết tới quest đó.
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
  private matFsyncKeTiep = false;
  private readonly tornSector = new Map<number, number>();

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
    if (this.matFsyncKeTiep) {
      this.matFsyncKeTiep = false;
      return; // "nói dối": không đẩy gì cả, cache vẫn còn nguyên — read() vẫn đúng, crash() sẽ lộ.
    }
    for (const [sector, data] of this.cache) {
      const soByteThanhCong = this.tornSector.get(sector);
      if (soByteThanhCong === undefined) {
        this.platter.set(sector, data);
        continue;
      }
      this.tornSector.delete(sector);
      const cu = this.platter.get(sector) ?? new Uint8Array(this.kichThuocSector);
      const ghiDuoc = new Uint8Array(this.kichThuocSector);
      ghiDuoc.set(data.slice(0, soByteThanhCong), 0);
      ghiDuoc.set(cu.slice(soByteThanhCong), soByteThanhCong);
      this.platter.set(sector, ghiDuoc);
    }
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

  /**
   * Lần `fsync()` TIẾP THEO là no-op hoàn toàn — mô phỏng "lost fsync".
   * Tự tắt sau khi dùng (chỉ ảnh hưởng ĐÚNG một lần gọi `fsync()`).
   */
  boQuaFsyncKeTiep(): void {
    this.matFsyncKeTiep = true;
  }

  /**
   * Đánh dấu `sector`: lần `fsync()` TIẾP THEO đụng tới sector này chỉ ghi
   * thành công `soByteThanhCong` byte ĐẦU, phần còn lại giữ nguyên dữ liệu
   * CŨ — mô phỏng "torn write". Tự xoá đánh dấu sau khi dùng.
   */
  danhDauTornGhi(sector: number, soByteThanhCong: number): void {
    this.kiemTraSector(sector);
    if (
      !Number.isInteger(soByteThanhCong)
      || soByteThanhCong < 0
      || soByteThanhCong > this.kichThuocSector
    ) {
      throw new RangeError(
        `soByteThanhCong ${soByteThanhCong} phải trong [0, ${this.kichThuocSector}]`,
      );
    }
    this.tornSector.set(sector, soByteThanhCong);
  }
}
