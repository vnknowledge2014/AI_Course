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
}
