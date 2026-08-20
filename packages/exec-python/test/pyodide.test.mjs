import { test, before } from 'node:test';
import assert from 'node:assert/strict';
import { chayTrongWorkerPython } from '../dist/worker-body.js';
import { chanDoanTuTraceback } from '../dist/index.js';

/** Chạy CPython THẬT qua Pyodide.
 *
 *  Test này cố tình không giả lập. Toàn bộ lý do chọn Pyodide thay vì tự viết
 *  interpreter là để người học gặp đúng thông báo lỗi và đúng hành vi họ sẽ
 *  gặp bên ngoài ứng dụng — một bản giả lập trong test sẽ xoá mất chính điều
 *  đang cần kiểm.
 */
let py;

before(async () => {
  const { loadPyodide } = await import('pyodide');
  py = await loadPyodide();
}, { timeout: 120_000 });

const chay = (ma, maKiemTra) =>
  chayTrongWorkerPython({ loai: 'chay', id: 1, ma, ...(maKiemTra ? { maKiemTra } : {}) }, py);

test('in ra được một câu', () => {
  const r = chay('print("Xin chào")');
  assert.equal(r.ok, true);
  assert.equal(r.xuat.trim(), 'Xin chào');
});

test('bài vàng: print("2 + 3") in ra nguyên văn, không tính', () => {
  // Chính là điều bước `predict` của bài đầu tiên dạy. Nếu engine tự tính
  // thành 5 thì bài học nói dối, và đó là kiểu sai tệ nhất một học liệu mắc.
  assert.equal(chay('print("2 + 3")').xuat.trim(), '2 + 3');
  assert.equal(chay('print(2 + 3)').xuat.trim(), '5');
});

test('mã kiểm tra chạy chung không gian tên với mã người học', () => {
  const r = chay('def gap_doi(x):\n    return x * 2', 'assert gap_doi(21) == 42');
  assert.equal(r.ok, true, r.loi ?? '');
});

test('mã kiểm tra sai thì báo trượt, không báo đạt', () => {
  const r = chay('def gap_doi(x):\n    return x + 2', 'assert gap_doi(21) == 42');
  assert.equal(r.ok, false);
  assert.match(r.loi, /AssertionError/);
});

test('output in ra trước khi lỗi xảy ra vẫn được giữ', () => {
  // Người học cần thấy chương trình chạy tới đâu thì hỏng. Vứt output đi là
  // lấy mất manh mối duy nhất họ có.
  const r = chay('print("dòng một")\nprint("dòng hai")\nprint(1 / 0)');
  assert.equal(r.ok, false);
  assert.match(r.xuat, /dòng một/);
  assert.match(r.xuat, /dòng hai/);
});

test('TypeError của bài 16 dịch được sang tiếng Việt', () => {
  const r = chay('so_to = 3\nprint("Số tô: " + so_to)');
  assert.equal(r.ok, false);
  const c = chanDoanTuTraceback(r.loi);
  assert.equal(c.muc, 'loi');
  assert.ok(c.thongDiep.length > 0);
  assert.ok(c.viSao && c.viSao.length > 0, 'phải có tầng "vì sao", không chỉ nhắc lại lỗi');
});

test('NameError khi dùng tên không có nháy — đúng bài 8', () => {
  const r = chay('print(Phở)');
  assert.equal(r.ok, false);
  assert.match(r.loi, /NameError/);
});
