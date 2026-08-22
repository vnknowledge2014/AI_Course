/**
 * Thế giới `number-line` — thanh số cho Realm 2.
 *
 * Ba bài của T2.1 dạy đúng thứ chỉ nhìn mới hiểu: cộng là BƯỚC sang phải,
 * nhân là KÉO GIÃN cả thanh, nhân số âm là LẬT thanh quanh mốc 0. Đọc câu
 * "(−1) × (−1) = 1" thì phải tin; thấy thanh số lật hai lần rồi về chỗ cũ thì
 * không phải tin nữa.
 *
 * Cùng kiến trúc với `grid-bot`: mã người học chỉ ghi `TraceEvent`, giao diện
 * phát lại. Lý do vẫn thế — cho mã người học chạm vào bộ vẽ thì một vòng lặp
 * vô hạn của họ treo luôn chỗ duy nhất còn nút Dừng.
 *
 * Khác `grid-bot` ở một điểm quan trọng: ô lưới rời rạc, còn thanh số LIÊN
 * TỤC. Vị trí giữ bằng số thực để `nhan(0.5)` hay `di(1/3)` nói được điều
 * chúng phải nói — mà đó chính là chỗ phân số và số thập phân bước vào.
 */

import type { Pyodide } from './worker-body.js';
import type { SuKien } from './the-gioi.js';

export interface CauHinhThanhSo {
  /** Hai đầu thanh số nhìn thấy được. */
  tu: number;
  den: number;
  /** Chỗ đứng ban đầu. */
  bat_dau: number;
  /** Đích cần tới. Bỏ trống thì bài chỉ để nghịch, không có thắng thua. */
  dich?: number;
  /** Sai số cho phép khi so với đích — thanh số liên tục nên phải có. */
  sai_so?: number;
  /** Vạch chia hiện trên thanh. Mặc định mỗi đơn vị một vạch. */
  buoc_vach?: number;
}

export interface KetQuaThanhSo {
  su_kien: SuKien[];
  thang: boolean;
  vi_sao: string | null;
}

const THANH_SO_PY = `
import json

class _ThanhSo:
    def __init__(self, ch):
        self.tu = float(ch["tu"]); self.den = float(ch["den"])
        self.o = float(ch["bat_dau"])
        self.dich = ch.get("dich")
        self.sai_so = float(ch.get("sai_so", 1e-9))
        self.su_kien = []
        self.t = 0
        self.vi_sao = None
        self._ghi("alloc", o=self.o)

    def _ghi(self, kind, **p):
        self.t += 1
        self.su_kien.append({"t": self.t, "kind": kind, "payload": p, "depth": 0})

    def _kiem_bien(self):
        if self.o < self.tu or self.o > self.den:
            self.vi_sao = (
                f"Chỗ đứng {self._so(self.o)} đã ra ngoài thanh số "
                f"({self._so(self.tu)} tới {self._so(self.den)})."
            )
            self._ghi("panic", o=self.o)
            raise RuntimeError(self.vi_sao)

    @staticmethod
    def _so(v):
        # Số nguyên thì in không đuôi: thanh số dạy phép cộng mà hiện '3.0'
        # là kéo một khái niệm của Realm 1 vào chỗ chưa cần tới nó.
        return str(int(v)) if float(v).is_integer() else str(round(float(v), 6))

    def di(self, khoang):
        """Bước sang phải \`khoang\` đơn vị. Số âm thì lùi lại."""
        cu = self.o
        self.o = cu + float(khoang)
        self._ghi("move", tu=cu, den=self.o, khoang=float(khoang))
        self._kiem_bien()

    def nhay_toi(self, cho):
        cu = self.o
        self.o = float(cho)
        self._ghi("move", tu=cu, den=self.o, nhay=True)
        self._kiem_bien()

    def nhan(self, he_so):
        """Kéo giãn thanh số quanh mốc 0. Hệ số âm thì LẬT."""
        cu = self.o
        self.o = cu * float(he_so)
        self._ghi("borrow", tu=cu, den=self.o, he_so=float(he_so))
        self._kiem_bien()

    def dang_o(self):
        """Chỗ đang đứng. Trả \`int\` khi giá trị nguyên.

        Vị trí giữ bằng số thực để phân số nói được điều nó phải nói, nhưng
        \`noi(dang_o())\` mà in ra '3.0' là kéo khái niệm float của Realm 1 vào
        một bài toán chưa cần tới nó — và người học sẽ hỏi cái đuôi ấy ở đâu
        ra, đúng lúc bài đang nói về chuyện khác.
        """
        return int(self.o) if float(self.o).is_integer() else self.o

    def noi(self, cau):
        self._ghi("print", cau=str(cau))

_ts = _ThanhSo(json.loads(_cau_hinh))
di = _ts.di
nhay_toi = _ts.nhay_toi
nhan = _ts.nhan
dang_o = _ts.dang_o
noi = _ts.noi
`;

export function chayTrenThanhSo(
  py: Pyodide,
  ma: string,
  cau_hinh: CauHinhThanhSo,
): KetQuaThanhSo {
  py.globals.set('_cau_hinh', JSON.stringify(cau_hinh));
  py.runPython(THANH_SO_PY);

  let ngoai_le: string | null = null;
  try {
    py.runPython(ma);
  } catch (e) {
    const m = e instanceof Error ? e.message : String(e);
    ngoai_le = m.includes('ra ngoài thanh số') ? null : m;
  }

  py.runPython(
    '_kq = json.dumps({"su_kien": _ts.su_kien, "o": _ts.o, "vi_sao": _ts.vi_sao})',
  );
  const tho = JSON.parse(String(py.globals.get('_kq'))) as {
    su_kien: SuKien[];
    o: number;
    vi_sao: string | null;
  };

  // Lỗi trong mã người học (chia cho 0, tên sai) KHÔNG phải "thua trong thế
  // giới" — để tầng `run` dịch nó, đừng bọc lại thành một câu về thanh số.
  if (ngoai_le !== null) {
    return { su_kien: tho.su_kien, thang: false, vi_sao: null };
  }

  const dich = cau_hinh.dich;
  if (tho.vi_sao !== null) {
    return { su_kien: tho.su_kien, thang: false, vi_sao: tho.vi_sao };
  }
  if (dich === undefined) {
    // Không có đích ⇒ sân chơi, không có thắng thua.
    return { su_kien: tho.su_kien, thang: true, vi_sao: null };
  }

  const sai_so = cau_hinh.sai_so ?? 1e-9;
  const lech = Math.abs(tho.o - dich);
  const so = (v: number) => (Number.isInteger(v) ? String(v) : String(Math.round(v * 1e6) / 1e6));
  return {
    su_kien: tho.su_kien,
    thang: lech <= sai_so,
    vi_sao: lech <= sai_so ? null : `Dừng ở ${so(tho.o)}, đích là ${so(dich)}.`,
  };
}
