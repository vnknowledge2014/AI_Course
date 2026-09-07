/**
 * Bộ thực thi Rust — nạp `byte_rust.wasm` (xem ADR-001).
 *
 * Module WASM này tự nó đã an toàn: nó có ngân sách nhiên liệu và giới hạn độ
 * sâu bên trong, nên vòng lặp vô hạn của người học được chặn ở tầng Rust chứ
 * không cần host can thiệp. Nhờ vậy engine này KHÔNG dùng Worker/`CongWorker`
 * như hai engine còn lại — `chay()` chạy đồng bộ ngay trên luồng gọi nó (xem
 * `apps/byte/src/lib/chay_ma.ts`, hàm `bo_thuc_thi_rust`). Chỉ cần bọc bên
 * ngoài trong Worker nếu sau này người dùng thấy UI khựng lúc nạp/chạy WASM.
 */

import type { BoThucThi, KetQuaChay, KetQuaThoRust, TuyChonChay } from '@byte/exec-core';
import { doiChanDoanRust } from '@byte/exec-core';

interface XuatWasm {
  memory: WebAssembly.Memory;
  br_cap_phat(len: number): number;
  br_giai_phong(ptr: number, len: number): void;
  br_chay(ptr: number, len: number): bigint;
}

export class BoThucThiRust implements BoThucThi {
  readonly ngonNgu = 'rust' as const;

  private xuat: XuatWasm | null = null;
  private dangNap: Promise<void> | null = null;
  private readonly encoder = new TextEncoder();
  private readonly decoder = new TextDecoder();

  /**
   * @param layWasm Hàm trả về bytecode của module. Nhận vào dạng hàm thay vì
   *   URL để engine dùng được ở mọi nơi: `fetch` trên web, `readFile` trên
   *   desktop, asset bundle trên mobile.
   */
  constructor(private readonly layWasm: () => Promise<BufferSource>) {}

  async sanSang(): Promise<void> {
    if (this.xuat) return;
    if (!this.dangNap) {
      this.dangNap = (async () => {
        const bytes = await this.layWasm();
        const { instance } = await WebAssembly.instantiate(bytes, {});
        this.xuat = instance.exports as unknown as XuatWasm;
      })();
    }
    await this.dangNap;
  }

  async chay(ma: string, tuyChon: TuyChonChay = {}): Promise<KetQuaChay> {
    await this.sanSang();
    const w = this.xuat;
    if (!w) throw new Error('module WASM chưa nạp được');

    const nguon = tuyChon.maKiemTra ? `${ma}\n${tuyChon.maKiemTra}` : ma;
    const buf = this.encoder.encode(nguon);
    const t0 = performance.now();

    const inPtr = w.br_cap_phat(buf.length);
    let outPtr = 0;
    let outLen = 0;
    try {
      new Uint8Array(w.memory.buffer, inPtr, buf.length).set(buf);
      const goi = w.br_chay(inPtr, buf.length);
      outPtr = Number(goi >> 32n);
      outLen = Number(goi & 0xffffffffn);
      const json = this.decoder.decode(new Uint8Array(w.memory.buffer, outPtr, outLen));
      return ketQuaTuThoRust(JSON.parse(json) as KetQuaThoRust, performance.now() - t0);
    } finally {
      // Giải phóng kể cả khi JSON.parse ném — nếu không, mỗi lần lỗi là một lần rò rỉ.
      if (outLen > 0) w.br_giai_phong(outPtr, outLen);
      w.br_giai_phong(inPtr, buf.length);
    }
  }

  dong(): void {
    this.xuat = null;
    this.dangNap = null;
  }
}

/** Đổi JSON thô của `byte-rust` thành `KetQuaChay` — khúc diễn dịch CHUNG của
 *  cả hai host chạy Rust của app:
 *
 *  - Đường WASM: [`BoThucThiRust.chay`] vừa gọi ở trên.
 *  - Đường Tauri native: command `chay_rust` ở `apps/byte/src-tauri/src/lib.rs`
 *    trả về ĐÚNG cùng một chuỗi JSON (gọi thẳng
 *    `byte_rust::wasm::chay_thanh_json` — một hàm, một định dạng).
 *
 *  Cùng một hàm diễn dịch là điều kiện bắt buộc: một bài ĐẠT trên Mac mà
 *  TRƯỢT trên Android chỉ vì hai host map kết quả khác nhau thì người học
 *  không còn tin vào công cụ nữa.
 */
export function ketQuaTuThoRust(tho: KetQuaThoRust, thoiGianMs: number): KetQuaChay {
  const chanDoan = tho.chan_doan.map(doiChanDoanRust);
  return {
    ok: tho.ok,
    xuat: tho.xuat,
    chanDoan,
    thoiGianMs,
    // BR0500 = hết nhiên liệu, BR0505 = đệ quy quá sâu.
    biNgat: chanDoan.some((d) => d.ma === 'BR0500' || d.ma === 'BR0505'),
  };
}
