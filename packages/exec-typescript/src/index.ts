/**
 * Bộ thực thi TypeScript.
 *
 * Engine cũ (`fp/game/src/engine/runner.ts`) gọi `new Function(code)()` ngay
 * trên luồng giao diện. Người học viết `while (true) {}` là cả app đứng im,
 * không nút nào bấm được, phải tải lại trang và mất bài làm.
 *
 * Ở đây mã chạy trong Worker và host giữ đồng hồ. Hết giờ thì host **giết**
 * worker — cách duy nhất chắc chắn ngắt được vòng lặp trong JavaScript.
 */

import type {
  BoThucThi,
  ChanDoan,
  CongWorker,
  KetQuaChay,
  TinNhanTuWorker,
  TuyChonChay,
} from '@byte/exec-core';
import { chanDoanQuaGio } from '@byte/exec-core';

const HET_HAN_MAC_DINH_MS = 5000;

export class BoThucThiTypeScript implements BoThucThi {
  readonly ngonNgu = 'typescript' as const;

  private cong: CongWorker | null = null;
  private idTiepTheo = 1;

  /** @param taoWorker Tạo worker mới. Gọi lại mỗi khi worker cũ bị giết. */
  constructor(private readonly taoWorker: () => CongWorker) {}

  async sanSang(): Promise<void> {
    this.bacDamBaoCong();
  }

  private bacDamBaoCong(): CongWorker {
    if (!this.cong) {
      this.cong = this.taoWorker();
    }
    return this.cong;
  }

  async chay(ma: string, tuyChon: TuyChonChay = {}): Promise<KetQuaChay> {
    const hetHan = tuyChon.hetHanMs ?? HET_HAN_MAC_DINH_MS;
    const cong = this.bacDamBaoCong();
    const id = this.idTiepTheo++;
    const t0 = performance.now();

    const phanHoi = await new Promise<TinNhanTuWorker | null>((resolve) => {
      const dongHo = setTimeout(() => {
        // Giết worker: cách DUY NHẤT ngắt được vòng lặp vô hạn trong JS.
        cong.giet();
        this.cong = null;
        resolve(null);
      }, hetHan);

      cong.khiNhan((tin) => {
        if (tin.loai === 'xong' && tin.id === id) {
          clearTimeout(dongHo);
          resolve(tin);
        }
      });

      cong.gui({ loai: 'chay', id, ma, maKiemTra: tuyChon.maKiemTra });
    });

    const thoiGianMs = performance.now() - t0;

    if (phanHoi === null) {
      return {
        ok: false,
        xuat: '',
        chanDoan: [chanDoanQuaGio(hetHan)],
        thoiGianMs,
        biNgat: true,
      };
    }
    if (phanHoi.loai !== 'xong') {
      return { ok: false, xuat: '', chanDoan: [], thoiGianMs, biNgat: false };
    }
    return {
      ok: phanHoi.ok,
      xuat: phanHoi.xuat,
      chanDoan: phanHoi.loi ? [chanDoanTuLoiJs(phanHoi.loi)] : [],
      thoiGianMs,
      biNgat: false,
    };
  }

  dong(): void {
    this.cong?.giet();
    this.cong = null;
  }
}

/**
 * Đổi lỗi runtime của JavaScript thành chẩn đoán ba tầng.
 *
 * Thông báo gốc của V8 viết cho lập trình viên đã biết JS. Câu
 * `undefined is not a function` không nói cho người mới biết phải làm gì.
 */
export function chanDoanTuLoiJs(loi: string): ChanDoan {
  const khop = /^(\w*Error): ?(.*)$/s.exec(loi.trim());
  const ten = khop?.[1] ?? 'Error';
  const noiDung = khop?.[2] ?? loi;

  let viSao: string | null = null;
  let cachSua: string[] = [];
  let khaiNiem: string | null = null;
  let ma = 'TS0599';

  if (ten === 'ReferenceError') {
    ma = 'TS0501';
    viSao = 'tên này chưa được khai báo, hoặc bị gõ sai một chữ';
    cachSua = ['kiểm tra lại chính tả', 'khai báo trước khi dùng: `const ten = ...`'];
    khaiNiem = 'biến';
  } else if (ten === 'TypeError' && /is not a function/.test(noiDung)) {
    ma = 'TS0502';
    viSao = 'bạn đang gọi một thứ không phải hàm — thường do gõ sai tên phương thức';
    cachSua = ['kiểm tra tên phương thức', 'kiểm tra giá trị có đúng kiểu bạn nghĩ không'];
    khaiNiem = 'hàm';
  } else if (ten === 'TypeError' && /(undefined|null)/.test(noiDung)) {
    ma = 'TS0503';
    viSao = 'giá trị đang là `undefined` hoặc `null`, nên không lấy được thuộc tính bên trong';
    cachSua = ['kiểm tra trước khi dùng: `if (x) { ... }`', 'dùng `x?.thuocTinh` để an toàn'];
    khaiNiem = 'undefined và null';
  } else if (ten === 'RangeError') {
    ma = 'TS0504';
    viSao = 'nhiều khả năng hàm gọi lại chính nó mà không có điều kiện dừng';
    cachSua = ['thêm trường hợp cơ sở, ví dụ `if (n <= 1) return 1;`'];
    khaiNiem = 'đệ quy';
  } else if (ten === 'SyntaxError') {
    ma = 'TS0505';
    viSao = 'mã không đúng cú pháp nên chưa chạy được dòng nào';
    cachSua = ['kiểm tra dấu ngoặc, dấu nháy và dấu chấm phẩy có khớp không'];
    khaiNiem = 'cú pháp cơ bản';
  }

  return {
    ma,
    muc: 'loi',
    thongDiep: noiDung,
    dong: 1,
    cot: 1,
    doDai: 1,
    viSao,
    cachSua,
    khaiNiem,
    vanBan: loi,
  };
}

export { chayTrongWorker } from './worker-body.js';
export type { BoChuyenDoi } from './worker-body.js';
