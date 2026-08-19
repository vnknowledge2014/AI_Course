/**
 * Bộ thực thi Python — chạy CPython thật qua Pyodide (biên dịch WASM).
 *
 * Khác Rust và TypeScript, ở đây ta KHÔNG tự viết interpreter: Pyodide là
 * CPython thật, nên ngữ nghĩa đúng tuyệt đối và người học dùng được cả thư
 * viện chuẩn. Cái giá là 13 MB — lớn hơn interpreter Rust 42 lần — nên nó được
 * nạp lười, chỉ khi người học thực sự mở bài Python đầu tiên.
 *
 * Cũng như TypeScript, mã chạy trong Worker và bị giết khi quá giờ: Python
 * chạy trong WASM không tự ngắt được từ bên trong.
 */

import type { BoThucThi, ChanDoan, KetQuaChay, TuyChonChay } from './types.js';
import type { CongWorker } from './typescript.js';
import { chanDoanQuaGio } from './typescript.js';
import type { TinNhanTuWorker } from './worker-protocol.js';

const HET_HAN_MAC_DINH_MS = 15000; // Pyodide khởi động chậm hơn nhiều

export class BoThucThiPython implements BoThucThi {
  readonly ngonNgu = 'python' as const;

  private cong: CongWorker | null = null;
  private idTiepTheo = 1;
  private daSanSang = false;

  constructor(private readonly taoWorker: () => CongWorker) {}

  async sanSang(): Promise<void> {
    if (this.daSanSang) return;
    const cong = this.bacDamBaoCong();
    await new Promise<void>((resolve) => {
      cong.khiNhan((tin) => {
        if (tin.loai === 'san_sang') resolve();
      });
      // Worker gửi `san_sang` ngay khi Pyodide nạp xong.
    });
    this.daSanSang = true;
  }

  private bacDamBaoCong(): CongWorker {
    if (!this.cong) {
      this.cong = this.taoWorker();
      this.daSanSang = false;
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
        cong.giet();
        this.cong = null;
        this.daSanSang = false;
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
      return { ok: false, xuat: '', chanDoan: [chanDoanQuaGio(hetHan)], thoiGianMs, biNgat: true };
    }
    if (phanHoi.loai !== 'xong') {
      return { ok: false, xuat: '', chanDoan: [], thoiGianMs, biNgat: false };
    }
    return {
      ok: phanHoi.ok,
      xuat: phanHoi.xuat,
      chanDoan: phanHoi.loi ? [chanDoanTuTraceback(phanHoi.loi)] : [],
      thoiGianMs,
      biNgat: false,
    };
  }

  dong(): void {
    this.cong?.giet();
    this.cong = null;
    this.daSanSang = false;
  }
}

/**
 * Đổi traceback của Python thành chẩn đoán ba tầng.
 *
 * Traceback gốc bắt đầu bằng nhiều dòng khung stack của chính Pyodide, mà người
 * mới học không hiểu và cũng không cần. Ta chỉ giữ dòng lỗi cuối, lấy số dòng
 * trong mã của người học, rồi bổ sung giải thích và cách sửa.
 */
export function chanDoanTuTraceback(traceback: string): ChanDoan {
  const dongs = traceback.trimEnd().split('\n');
  const dongCuoi = dongs[dongs.length - 1] ?? traceback;
  const khop = /^([A-Za-z_]*(?:Error|Exception|Interrupt)): ?(.*)$/s.exec(dongCuoi.trim());
  const ten = khop?.[1] ?? 'Error';
  const noiDung = khop?.[2] ?? dongCuoi.trim();

  // Số dòng: lấy lần khớp `line N` CUỐI CÙNG — đó là mã của người học,
  // các lần trước là khung của Pyodide.
  let dong = 1;
  const soDong = [...traceback.matchAll(/line (\d+)/g)];
  const cuoi = soDong[soDong.length - 1]?.[1];
  if (cuoi) dong = Number.parseInt(cuoi, 10) || 1;

  let ma = 'PY0599';
  let viSao: string | null = null;
  let cachSua: string[] = [];
  let khaiNiem: string | null = null;

  switch (ten) {
    case 'NameError':
      ma = 'PY0501';
      viSao = 'tên này chưa được gán giá trị, hoặc bị gõ sai một chữ';
      cachSua = ['kiểm tra lại chính tả', 'gán giá trị trước khi dùng: `ten = ...`'];
      khaiNiem = 'biến';
      break;
    case 'TypeError':
      ma = 'PY0502';
      viSao = 'phép toán hoặc lời gọi đang nhận sai kiểu dữ liệu';
      cachSua = [
        'kiểm tra kiểu bằng `type(x)`',
        'Python không tự cộng số với chuỗi — dùng `f"{a}{b}"` hoặc `str(a) + b`',
      ];
      khaiNiem = 'kiểu dữ liệu';
      break;
    case 'IndexError':
      ma = 'PY0503';
      viSao = 'phần tử đánh số từ 0, nên danh sách n phần tử chỉ có chỉ số 0 tới n-1';
      cachSua = ['kiểm tra độ dài bằng `len(x)` trước khi truy cập'];
      khaiNiem = 'chỉ số';
      break;
    case 'KeyError':
      ma = 'PY0504';
      viSao = 'khoá này không có trong dict';
      cachSua = ['dùng `d.get(khoa)` để nhận `None` thay vì lỗi', 'kiểm tra bằng `khoa in d`'];
      khaiNiem = 'dict';
      break;
    case 'ZeroDivisionError':
      ma = 'PY0505';
      viSao = 'chia cho 0 không có kết quả trong toán học';
      cachSua = ['kiểm tra mẫu số trước: `if b != 0:`'];
      khaiNiem = 'phép chia';
      break;
    case 'IndentationError':
    case 'TabError':
      ma = 'PY0506';
      viSao =
        'Python dùng khoảng thụt đầu dòng để biết đoạn nào nằm trong đoạn nào — thụt sai là đổi cả ý nghĩa chương trình';
      cachSua = [
        'dùng nhất quán 4 dấu cách cho mỗi cấp',
        'không trộn lẫn Tab với dấu cách trong cùng một file',
      ];
      khaiNiem = 'thụt lề';
      break;
    case 'SyntaxError':
      ma = 'PY0507';
      viSao = 'mã không đúng cú pháp nên chưa chạy được dòng nào';
      cachSua = ['kiểm tra dấu hai chấm `:` ở cuối `if`, `for`, `def`', 'kiểm tra ngoặc có khớp không'];
      khaiNiem = 'cú pháp cơ bản';
      break;
    case 'AttributeError':
      ma = 'PY0508';
      viSao = 'đối tượng này không có thuộc tính hay phương thức đó';
      cachSua = ['kiểm tra chính tả', 'xem đối tượng có gì bằng `dir(x)`'];
      khaiNiem = 'đối tượng';
      break;
    case 'RecursionError':
      ma = 'PY0509';
      viSao = 'nhiều khả năng hàm gọi lại chính nó mà không có điều kiện dừng';
      cachSua = ['thêm trường hợp cơ sở, ví dụ `if n <= 1: return 1`'];
      khaiNiem = 'đệ quy';
      break;
    case 'AssertionError':
      ma = 'PY0510';
      viSao = '`assert` nói "chỗ này chắc chắn phải đúng"; nó không đúng nên chương trình dừng';
      khaiNiem = 'kiểm thử';
      break;
  }

  return {
    ma,
    muc: 'loi',
    thongDiep: noiDung || ten,
    dong,
    cot: 1,
    doDai: 1,
    viSao,
    cachSua,
    khaiNiem,
    vanBan: traceback,
  };
}
