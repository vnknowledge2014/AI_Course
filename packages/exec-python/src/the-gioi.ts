/**
 * Thế giới `grid-bot` — Byte đi trên lưới theo mã người học viết.
 *
 * Đây là chỗ học liệu này khác một cuốn sách: người học gõ `di_toi()` rồi
 * THẤY Byte bước đi. Sai một bước là thấy ngay nó đâm vào tường, không phải
 * đọc một dòng chữ nói rằng đã sai.
 *
 * Mã người học KHÔNG BAO GIỜ vẽ gì cả — nó chỉ gọi lệnh, và mỗi lệnh ghi lại
 * một `TraceEvent`. Giao diện đọc chuỗi sự kiện ấy rồi dựng hình. Tách như vậy
 * là bắt buộc: nếu mã người học chạm được vào bộ vẽ thì một vòng lặp vô hạn
 * trong bài của họ sẽ treo luôn giao diện, mà giao diện lại là thứ duy nhất
 * còn nút Dừng.
 *
 * Hợp đồng `TraceEvent` do MASTERPLAN §4 định, không phải do file này đặt ra.
 */

import type { Pyodide } from './worker-body.js';

export interface SuKien {
  t: number;
  kind: string;
  payload: Record<string, unknown>;
  depth: number;
}

export interface CauHinhLuoi {
  rong: number;
  cao: number;
  /** Ô xuất phát của Byte. */
  bat_dau: { x: number; y: number };
  /** Hướng ban đầu: 0 = phải, 1 = xuống, 2 = trái, 3 = lên. */
  huong?: number;
  /** Ô không đi qua được. */
  tuong?: { x: number; y: number }[];
  /** Ô cần nhặt hết để thắng. */
  vien?: { x: number; y: number }[];
}

export interface KetQuaLuoi {
  su_kien: SuKien[];
  /** Nhặt hết viên và không đâm tường. */
  thang: boolean;
  /** Lý do thua, để Byte nói ra bằng tiếng Việt. */
  vi_sao: string | null;
}

/** Mô-đun Python dựng sẵn, nạp TRƯỚC mã người học. */
const THE_GIOI_PY = `
import json

class _Luoi:
    def __init__(self, ch):
        self.rong = ch["rong"]; self.cao = ch["cao"]
        self.x = ch["bat_dau"]["x"]; self.y = ch["bat_dau"]["y"]
        self.huong = ch.get("huong", 0)
        self.tuong = {(o["x"], o["y"]) for o in ch.get("tuong", [])}
        self.vien = {(o["x"], o["y"]) for o in ch.get("vien", [])}
        self.da_nhat = set()
        self.su_kien = []
        self.t = 0
        self.vi_sao = None
        self._ghi("alloc", x=self.x, y=self.y, huong=self.huong)

    def _ghi(self, kind, **p):
        self.t += 1
        self.su_kien.append({"t": self.t, "kind": kind, "payload": p, "depth": 0})

    def _chan(self, x, y):
        return not (0 <= x < self.rong and 0 <= y < self.cao) or (x, y) in self.tuong

    def di_toi(self, so_o=1):
        dx, dy = [(1, 0), (0, 1), (-1, 0), (0, -1)][self.huong % 4]
        for _ in range(so_o):
            nx, ny = self.x + dx, self.y + dy
            if self._chan(nx, ny):
                self._ghi("panic", ly_do="tuong", x=nx, y=ny)
                # Dừng bằng ngoại lệ chứ không im lặng: người học phải THẤY
                # chương trình hỏng ở đâu, và tầng \`run\` báo nó bằng tiếng Việt.
                if self.vi_sao is None:
                    self.vi_sao = "Byte đâm vào tường — ô đó không đi qua được."
                raise RuntimeError(self.vi_sao)
            self.x, self.y = nx, ny
            self._ghi("move", x=self.x, y=self.y)
            if (self.x, self.y) in self.vien and (self.x, self.y) not in self.da_nhat:
                self.da_nhat.add((self.x, self.y))
                self._ghi("retrieve", x=self.x, y=self.y)

    def quay_phai(self):
        self.huong = (self.huong + 1) % 4
        self._ghi("assign", huong=self.huong)

    def quay_trai(self):
        self.huong = (self.huong + 3) % 4
        self._ghi("assign", huong=self.huong)

    def noi(self, cau):
        self._ghi("print", cau=str(cau))

    def phia_truoc_trong(self):
        dx, dy = [(1, 0), (0, 1), (-1, 0), (0, -1)][self.huong % 4]
        return not self._chan(self.x + dx, self.y + dy)

    def con_vien(self):
        return len(self.vien - self.da_nhat) > 0

_luoi = _Luoi(json.loads(_cau_hinh))
di_toi = _luoi.di_toi
quay_phai = _luoi.quay_phai
quay_trai = _luoi.quay_trai
noi = _luoi.noi
phia_truoc_trong = _luoi.phia_truoc_trong
con_vien = _luoi.con_vien
`;

/**
 * Chạy mã người học trong thế giới lưới, trả về chuỗi sự kiện để giao diện dựng.
 *
 * `py` phải là một Pyodide đã nạp xong. Trạng thái lưới sống trong Python nên
 * mọi lệnh đều đi qua đúng một nguồn sự thật.
 */
export function chayTrenLuoi(py: Pyodide, ma: string, cau_hinh: CauHinhLuoi): KetQuaLuoi {
  py.globals.set('_cau_hinh', JSON.stringify(cau_hinh));
  py.runPython(THE_GIOI_PY);

  let vi_sao: string | null = null;
  try {
    py.runPython(ma);
  } catch (e) {
    const m = e instanceof Error ? e.message : String(e);
    // Ngoại lệ do chính thế giới ném ra đã có câu tiếng Việt; ngoại lệ khác là
    // lỗi trong mã người học và tầng `run` sẽ dịch nó.
    vi_sao = m.includes('đâm vào tường')
      ? 'Byte đâm vào tường — ô đó không đi qua được.'
      : null;
  }

  py.runPython(
    '_kq = json.dumps({"su_kien": _luoi.su_kien, ' +
      '"con_lai": len(_luoi.vien - _luoi.da_nhat), ' +
      '"vi_sao": _luoi.vi_sao})',
  );
  const tho = JSON.parse(String(py.globals.get('_kq'))) as {
    su_kien: SuKien[];
    con_lai: number;
    vi_sao: string | null;
  };

  const ly_do = vi_sao ?? tho.vi_sao;
  return {
    su_kien: tho.su_kien,
    thang: ly_do === null && tho.con_lai === 0,
    vi_sao:
      ly_do ??
      (tho.con_lai > 0 ? `Còn ${tho.con_lai} viên chưa nhặt.` : null),
  };
}
