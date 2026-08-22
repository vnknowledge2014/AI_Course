/// <reference lib="webworker" />
/**
 * Worker chạy TypeScript.
 *
 * Sống ở luồng riêng vì cùng lý do với Python: JavaScript không ngắt được một
 * vòng lặp từ bên trong, nên cách duy nhất chắc chắn dừng `while (true) {}` là
 * giết cả worker từ ngoài.
 *
 * Worker này chỉ CHẠY, không kiểm kiểu. Kiểm kiểu xảy ra ở host TRƯỚC khi gửi
 * xuống đây — mã đã biết chắc là sai thì không được chạy, và đó là luật chung
 * của cả ba engine (xem ADR-002).
 */
import { chayTrongWorker } from '@byte/exec-typescript';
import type { YeuCauChay } from '@byte/exec-core';
import { transform } from 'sucrase';

self.onmessage = (e: MessageEvent<YeuCauChay>) => {
  if (e.data?.loai !== 'chay') return;
  // Sucrase chỉ BÓC chú thích kiểu — nó không kiểm gì, và ở đây thế là đủ:
  // host đã kiểm kiểu bằng chính trình biên dịch `typescript` rồi. Dùng
  // Sucrase cho bước cuối vì nó nhanh hơn nhiều và không phải nạp lại 8,7 MB
  // vào worker.
  const kq = chayTrongWorker(e.data, (ma) => transform(ma, { transforms: ['typescript'] }).code);
  self.postMessage(kq);
};
