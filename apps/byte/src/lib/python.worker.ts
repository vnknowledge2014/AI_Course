/// <reference lib="webworker" />
/**
 * Worker chạy Python.
 *
 * Sống ở luồng riêng vì JavaScript không có cách nào ngắt một vòng lặp đang
 * chạy từ bên trong. Người mới học viết `while True:` là chuyện thường, và
 * cách duy nhất chắc chắn dừng được nó là GIẾT CẢ WORKER từ bên ngoài — điều
 * đó chỉ làm được khi mã của họ không nằm chung luồng với giao diện.
 */
import { gan, type Pyodide } from '@byte/exec-python';
import type { YeuCauChay } from '@byte/exec-core';

// Đường dẫn TƯƠNG ĐỐI với worker. Bản đóng gói Tauri phục vụ file từ đĩa qua
// `tauri://`, nên mọi đường dẫn tuyệt đối bắt đầu bằng `/` đều hỏng ở đó.
const GOC = new URL('../pyodide/', import.meta.url).href;

async function khoi_dong(): Promise<void> {
  const { loadPyodide } = (await import(/* @vite-ignore */ `${GOC}pyodide.mjs`)) as {
    loadPyodide: (o: { indexURL: string }) => Promise<Pyodide>;
  };
  const py = await loadPyodide({ indexURL: GOC });

  gan(
    py,
    (tin) => self.postMessage(tin),
    (xu_ly) => {
      self.onmessage = (e: MessageEvent<YeuCauChay>) => {
        if (e.data?.loai === 'chay') xu_ly(e.data);
      };
    },
  );
}

void khoi_dong();
