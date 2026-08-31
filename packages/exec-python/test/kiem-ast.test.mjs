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

// ── Sáu kind cho Realm 4 (FP) — cài lúc soạn T4.1 ──────────────────────────

test('frozen-dataclass đòi tường minh frozen=True, @dataclass trần không tính', () => {
  const co = `
from dataclasses import dataclass
@dataclass(frozen=True)
class Cfg:
    port: int
`;
  const khong = `
from dataclasses import dataclass
@dataclass
class Cfg:
    port: int
`;
  assert.equal(kiem(co, [q('frozen-dataclass', 'Cfg')]).dat, true);
  assert.equal(kiem(khong, [q('frozen-dataclass', 'Cfg')]).dat, false, '@dataclass trần mặc định frozen=False, không tính');
});

test('no-mutation bắt cả ba dạng: gọi phương thức sửa tại chỗ, gán vào ô/trường, del', () => {
  assert.equal(kiem('x = [1,2]\nx.append(3)', [q('no-mutation')]).dat, false);
  assert.equal(kiem('x = [1,2]\nx[0] = 9', [q('no-mutation')]).dat, false);
  assert.equal(kiem('cfg.port = 9', [q('no-mutation')]).dat, false);
  assert.equal(kiem('x = {1:2}\ndel x[1]', [q('no-mutation')]).dat, false);
  assert.equal(kiem('x = [1,2]\ny = x + [3]', [q('no-mutation')]).dat, true, 'tạo list MỚI không phải sửa tại chỗ');
  assert.equal(kiem('x = 1\nx = 2', [q('no-mutation')]).dat, true, 'gán lại một TÊN không phải sửa dữ liệu, chỉ đổi tên trỏ đi đâu');
});

test('no-global đảo chiều: có global/nonlocal là hỏng', () => {
  assert.equal(kiem('x = 1', [q('no-global')]).dat, true);
  assert.equal(kiem('def f():\n    global x\n    x = 1', [q('no-global')]).dat, false);
});

test('uses-generator chỉ đếm def có yield, không đếm generator expression', () => {
  const coYield = 'def dem():\n    yield 1\n    yield 2';
  const genExp = 'x = (i for i in range(3))';
  assert.equal(kiem(coYield, [q('uses-generator')]).dat, true);
  assert.equal(kiem(genExp, [q('uses-generator')]).dat, false, 'generator expression là comprehension, không phải uses-generator');
});

test('recursion đếm lời gọi tự thân, không đếm định nghĩa hàm hay gọi hàm khác', () => {
  const deQuy = 'def giai(n):\n    if n <= 1:\n        return 1\n    return n * giai(n - 1)';
  const khongDeQuy = 'def giai(n):\n    return n * 2\ndef goi():\n    return giai(3)';
  assert.equal(kiem(deQuy, [q('recursion', 'giai')]).dat, true);
  assert.equal(kiem(khongDeQuy, [q('recursion', 'giai')]).dat, false);
});

test('pure-fn (phủ định) bắt bốn dấu hiệu không thuần trong THÂN hàm, không xa hơn', () => {
  const thuan = 'def cong(a, b):\n    return a + b';
  const coGlobal = 'def f(x):\n    global y\n    return x + y';
  const coPrint = 'def f(x):\n    print(x)\n    return x';
  const coRandom = 'import random\ndef f():\n    return random.randint(1, 10)';
  const coSuaTaiCho = 'def f(x):\n    x.append(1)\n    return x';
  const hamKhac = 'def randint(a, b):\n    return a\ndef f():\n    return randint(1, 10)'; // trùng TÊN nhưng KHÔNG phải random.randint thật
  assert.equal(kiem(thuan, [q('pure-fn', 'cong')]).dat, true);
  assert.equal(kiem(coGlobal, [q('pure-fn', 'f')]).dat, false);
  assert.equal(kiem(coPrint, [q('pure-fn', 'f')]).dat, false);
  assert.equal(kiem(coRandom, [q('pure-fn', 'f')]).dat, false);
  assert.equal(kiem(coSuaTaiCho, [q('pure-fn', 'f')]).dat, false);
  assert.equal(kiem(hamKhac, [q('pure-fn', 'f')]).dat, true, 'gọi hàm tự viết trùng tên randint KHÔNG phải random.randint — không bắt oan');
});
