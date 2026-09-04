/**
 * Nạp một gói Pyodide (numpy, ...) đã VENDOR sẵn CỤC BỘ vào một instance
 * Pyodide đã có sẵn — dùng chung cho `kiem_ma_bai_hoc.mjs` VÀ
 * `kiem_dot_bien.mjs`, cả hai đều tự tạo `py` riêng qua `indexURL` MẶC ĐỊNH
 * (thư mục cài `pyodide` trong `node_modules`, KHÔNG có numpy).
 *
 * Gói npm `pyodide` chỉ mang CPython lõi + stdlib. numpy được `apps/byte/
 * scripts/nap-numpy.mjs` tải riêng (xác thực sha256 theo pyodide-lock.json)
 * VÀ đặt vào `apps/byte/public/pyodide/` — CÙNG thư mục runtime mà app THẬT
 * dùng (`apps/byte/src/lib/python.worker.ts`). Hàm này trỏ THẲNG tới file
 * wheel đó qua URL `file://` tuyệt đối — cách này nạp được BẤT KỂ `indexURL`
 * gốc của `py` đang trỏ đi đâu, không cần hai bên khớp thư mục.
 *
 * Tên gói ĐÚNG (`file_name` thật, không hardcode "numpy-2.2.5-...") được đọc
 * lại từ CHÍNH `pyodide-lock.json` trong thư mục vendor đó — nếu bản pyodide
 * ghim trong package.json đổi, tên file/sha256 đổi THEO, không cần sửa tay ở
 * đây.
 *
 * QUAN TRỌNG — "mồi" import TRƯỚC khi trả về: `kiem_ma_bai_hoc.mjs` VÀ
 * `kiem_dot_bien.mjs` đều chạy lời giải học sinh dưới một sandbox đếm bước
 * bằng `sys.settrace` (hạn mức 300.000 bước, chặn vòng lặp không dừng). ĐO
 * THẬT phát hiện: `loadPackage('numpy')` chỉ nạp WHEEL vào filesystem ảo —
 * CÂU LỆNH `import numpy` bên trong code học sinh vẫn phải tự chạy TOÀN BỘ
 * mã Python khởi tạo của numpy (hàng vạn dòng `__init__.py` cùng các
 * submodule) — VÀ `sys.settrace` đếm MỌI frame Python chạy qua, kể cả mã
 * NỘI BỘ của numpy, không chỉ mã học sinh. Lần `import numpy` ĐẦU TIÊN
 * trong một phiên tốn **283.455 bước** (đo thật) — gần hết 300.000 bước
 * hạn mức, chỉ còn ~16.500 bước cho toàn bộ logic bài học thật sự. Cách
 * sửa: chạy MỘT lần `import <tenGoi>` ở đây, NGOÀI mọi `sys.settrace` — sau
 * đó `<tenGoi>` đã nằm sẵn trong `sys.modules`, nên MỌI `import <tenGoi>`
 * sau đó (kể cả bên TRONG sandbox có trace) chỉ còn tốn **8 bước** (đã đo
 * lại, xác nhận). Không mồi trước thì MỌI bài học dùng numpy sẽ tự động
 * mất gần hết hạn mức bước chỉ để import — không liên quan gì tới độ khó
 * thật của bài.
 */
import { readFileSync, existsSync } from 'node:fs';
import { pathToFileURL } from 'node:url';

const THU_MUC_VENDOR = new URL('../apps/byte/public/pyodide/', import.meta.url);

/**
 * Nạp `tenGoi` (vd 'numpy') vào `py`. Ném lỗi rõ ràng, KHÔNG âm thầm bỏ qua,
 * nếu wheel chưa được vendor (chạy `pnpm run pyodide && pnpm run numpy` ở
 * `apps/byte/` trước khi chạy cổng này).
 */
export async function napGoiPyodideCucBo(py, tenGoi) {
  const lockPath = new URL('pyodide-lock.json', THU_MUC_VENDOR);
  if (!existsSync(lockPath)) {
    throw new Error(
      `napGoiPyodideCucBo: không thấy ${lockPath.pathname} — chạy 'pnpm run pyodide' trong apps/byte trước.`,
    );
  }
  const lock = JSON.parse(readFileSync(lockPath, 'utf-8'));
  const entry = lock.packages?.[tenGoi];
  if (!entry) {
    throw new Error(`napGoiPyodideCucBo: pyodide-lock.json không có gói '${tenGoi}'.`);
  }
  const wheelPath = new URL(entry.file_name, THU_MUC_VENDOR);
  if (!existsSync(wheelPath)) {
    throw new Error(
      `napGoiPyodideCucBo: không thấy ${wheelPath.pathname} — chạy 'pnpm run numpy' trong apps/byte trước.`,
    );
  }
  await py.loadPackage(pathToFileURL(wheelPath.pathname).href);
  // Mồi sys.modules — xem ghi chú "QUAN TRỌNG" ở trên. Chạy NGOÀI mọi
  // sys.settrace (hàm này luôn được gọi TRƯỚC khi sandbox đếm bước tồn tại).
  py.runPython(`import ${tenGoi}`);
}
