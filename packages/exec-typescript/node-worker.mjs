// Thân worker TypeScript cho ngữ cảnh Node (CLI/công cụ cổng), KHÔNG phải
// trình duyệt. `apps/byte/src/lib/typescript.worker.ts` là bản Web Worker
// cho app thật (dùng `self.onmessage`); file này là bản `node:worker_threads`
// song song, cho các công cụ trong `tools/` chạy TRỰC TIẾP trong Node —
// không có DOM, không `self`.
//
// Cùng logic lõi (`chayTrongWorker`) và cùng cách bóc kiểu (Sucrase) với bản
// trình duyệt — chỉ khác lớp vỏ giao tiếp (`parentPort` thay vì `postMessage`
// toàn cục). Đừng để hai bản lệch nhau: cả hai đều PHẢI chỉ chạy, không
// kiểm kiểu — kiểm kiểu xảy ra ở host TRƯỚC khi gửi xuống đây (xem ADR-002).
import { parentPort } from 'node:worker_threads';
import { transform } from 'sucrase';
import { chayTrongWorker } from './dist/index.js';

parentPort.on('message', (yeuCau) => {
  const phanHoi = chayTrongWorker(yeuCau, (ma) => transform(ma, { transforms: ['typescript'] }).code);
  parentPort.postMessage(phanHoi);
});
