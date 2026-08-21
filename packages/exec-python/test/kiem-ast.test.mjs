import { test, before } from 'node:test';
import assert from 'node:assert/strict';
import { kiemAst } from '../dist/kiem-ast.js';

let py;
before(async () => {
  const { loadPyodide } = await import('pyodide');
  py = await loadPyodide();
}, { timeout: 120_000 });

const q = (kind, target, min) => ({ lang: 'python', kind, ...(target ? { target } : {}), ...(min ? { min } : {}) });
const kiem = (ma, yeu = [], cam = []) => kiemAst(py, ma, yeu, cam);

test('uses-call bắt đúng tên hàm được gọi', () => {
  assert.equal(kiem('x = round(1.7)', [q('uses-call', 'round')]).dat, true);
  assert.equal(kiem('x = int(1.7)', [q('uses-call', 'round')]).dat, false);
});

test('uses-operator phân biệt // với /', () => {
  assert.equal(kiem('x = 7 // 2', [q('uses-operator', '//')]).dat, true);
  assert.equal(kiem('x = 7 / 2', [q('uses-operator', '//')]).dat, false);
});

/** Đây là lý do tồn tại của cả tầng `static`.
 *
 *  Bài dạy dấu ngoặc: starter `if ___:` với `du_tien = True`, `kip_gio =
 *  False`, chấm bằng một dòng output. `not (du_tien and kip_gio)` đạt — nhưng
 *  `True` cũng đạt, và `not du_tien and kip_gio` thì cho màn hình trống nên
 *  trượt vì lý do khác. Chỉ có AST mới hỏi được đúng câu: cặp ngoặc có ở đó
 *  không.
 */
test('nesting phân biệt `not (a and b)` với `not a and b`', () => {
  const yeu = [q('nesting', 'not/and')];
  assert.equal(kiem('r = not (a and b)', yeu).dat, true, 'có ngoặc thì đạt');
  assert.equal(kiem('r = not a and b', yeu).dat, false, 'không ngoặc thì trượt');
  assert.equal(kiem('r = True', yeu).dat, false, 'điền bừa thì trượt');
});

test('has-literal trong forbidAst chặn đáp án chép cứng', () => {
  const cam = [q('has-literal', '10000')];
  assert.equal(kiem('print(tien_con)', [], cam).dat, true);
  const r = kiem('print(10000)', [], cam);
  assert.equal(r.dat, false);
  assert.equal(r.cam.length, 1);
});

test('uses-fstring và uses-name', () => {
  assert.equal(kiem('print(f"con {tien}")', [q('uses-fstring'), q('uses-name', 'tien')]).dat, true);
  assert.equal(kiem('print("con " + str(tien))', [q('uses-fstring')]).dat, false);
});

test('min đếm được số lần xuất hiện', () => {
  // `tien = 0` là ĐẶT tên, không phải DÙNG tên — `uses-name` chỉ đếm chỗ đọc.
  assert.equal(kiem('a = 1\nb = 2', [q('uses-name', undefined, 1)]).dat, false, 'chỉ có gán thì chưa tham chiếu tên nào');
  assert.equal(kiem('a = 1\nprint(a)', [q('uses-name', 'a')]).dat, true, 'đọc `a` thì tính');
  assert.equal(kiem('print(a)\nprint(a)', [q('uses-call', 'print', 2)]).dat, true);
  assert.equal(kiem('print(a)', [q('uses-call', 'print', 2)]).dat, false);
});

test('no-import đảo chiều: có mặt là hỏng', () => {
  assert.equal(kiem('x = 1', [q('no-import')]).dat, true);
  assert.equal(kiem('import os', [q('no-import')]).dat, false);
});

test('mã sai cú pháp báo riêng, không nhầm thành sai hình dạng', () => {
  const r = kiem('if x', [q('uses-call', 'print')]);
  assert.equal(r.dat, false);
  assert.ok(r.loi_cu_phap, 'phải nói rõ là lỗi cú pháp — tầng `run` báo nó dễ hiểu hơn');
  assert.equal(r.thieu.length, 0, 'không kết luận thiếu hình dạng khi chưa parse nổi');
});
