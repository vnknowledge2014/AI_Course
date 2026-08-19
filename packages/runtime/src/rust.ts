/**
 * Bộ thực thi Rust — nạp `byte_rust.wasm` (xem ADR-001).
 *
 * Module WASM này tự nó đã an toàn: nó có ngân sách nhiên liệu và giới hạn độ
 * sâu bên trong, nên vòng lặp vô hạn của người học được chặn ở tầng Rust chứ
 * không cần host can thiệp. Nhờ vậy engine này không cần Worker để tránh treo
 * — nhưng vẫn chạy trong Worker để nhất quán với hai engine còn lại.
 */

import type { BoThucThi, KetQuaChay, KetQuaThoRust, TuyChonChay } from './types.js';
import { doiChanDoanRust } from './types.js';

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
      const tho = JSON.parse(json) as KetQuaThoRust;
      const chanDoan = tho.chan_doan.map(doiChanDoanRust);
      return {
        ok: tho.ok,
        xuat: tho.xuat,
        chanDoan,
        thoiGianMs: performance.now() - t0,
        // BR0500 = hết nhiên liệu, BR0505 = đệ quy quá sâu.
        biNgat: chanDoan.some((d) => d.ma === 'BR0500' || d.ma === 'BR0505'),
      };
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
