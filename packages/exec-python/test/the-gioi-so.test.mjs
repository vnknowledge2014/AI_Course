import { test, before } from 'node:test';
import assert from 'node:assert/strict';
import { chayTrenThanhSo } from '../dist/the-gioi-so.js';

let py;
before(async () => {
  const { loadPyodide } = await import('pyodide');
  py = await loadPyodide();
}, { timeout: 120_000 });

const TS = { tu: -10, den: 10, bat_dau: 0, dich: 6 };
const chay = (ma, ch = TS) => chayTrenThanhSo(py, ma, ch);

test('cộng là bước sang phải — đúng bài 13', () => {
  const r = chay('di(6)');
  assert.equal(r.thang, true, r.vi_sao ?? '');
  const b = r.su_kien.filter((s) => s.kind === 'move');
  assert.equal(b.length, 1);
  assert.deepEqual([b[0].payload.tu, b[0].payload.den], [0, 6]);
});

test('trừ là lùi lại — cùng một phép, khác dấu', () => {
  const r = chay('di(10)\ndi(-4)');
  assert.equal(r.thang, true, r.vi_sao ?? '');
});

/** Đây là lý do có thanh số: (−1)×(−1)=1 đọc thì phải tin, thấy thì không. */
test('nhân số âm là LẬT quanh mốc 0 — đúng bài 23', () => {
  const r = chayTrenThanhSo(py, 'di(3)\nnhan(-1)\nnhan(-1)', { tu: -10, den: 10, bat_dau: 0, dich: 3 });
  assert.equal(r.thang, true, r.vi_sao ?? '');
  const lat = r.su_kien.filter((s) => s.kind === 'borrow');
  assert.equal(lat.length, 2);
  assert.equal(lat[0].payload.den, -3, 'lật lần một đưa 3 sang -3');
  assert.equal(lat[1].payload.den, 3, 'lật lần hai đưa về chỗ cũ');
});

test('nhân là kéo giãn — đúng bài 22', () => {
  const r = chayTrenThanhSo(py, 'di(2)\nnhan(3)', { tu: -10, den: 10, bat_dau: 0, dich: 6 });
  assert.equal(r.thang, true, r.vi_sao ?? '');
});

test('thanh số LIÊN TỤC, không phải ô rời rạc', () => {
  // Phân số và thập phân phải nói được điều chúng phải nói, nếu không thì cả
  // nửa sau của T2.1 không có sân khấu nào dùng được.
  const r = chayTrenThanhSo(py, 'di(0.5)\ndi(0.25)', { tu: 0, den: 2, bat_dau: 0, dich: 0.75 });
  assert.equal(r.thang, true, r.vi_sao ?? '');
});

test('nhân 1/2 là co lại, không phải làm số nhỏ đi một cách bí ẩn', () => {
  const r = chayTrenThanhSo(py, 'di(8)\nnhan(0.5)', { tu: 0, den: 10, bat_dau: 0, dich: 4 });
  assert.equal(r.thang, true, r.vi_sao ?? '');
});

test('ra ngoài thanh số thì dừng và nói ra bằng tiếng Việt', () => {
  const r = chay('di(50)');
  assert.equal(r.thang, false);
  assert.match(r.vi_sao, /ra ngoài thanh số/);
  assert.ok(r.su_kien.some((s) => s.kind === 'panic'));
});

test('dừng sai chỗ thì nói rõ dừng ở đâu và đích ở đâu', () => {
  const r = chay('di(5)');
  assert.equal(r.thang, false);
  assert.match(r.vi_sao, /Dừng ở 5, đích là 6/);
});

test('lỗi trong mã người học KHÔNG bị bọc thành câu về thanh số', () => {
  // `1/0` là lỗi Python, và tầng `run` dịch nó tử tế hơn nhiều. Bọc lại thành
  // "bạn đi sai chỗ" là nói dối về nguyên nhân.
  const r = chay('di(1 / 0)');
  assert.equal(r.thang, false);
  assert.equal(r.vi_sao, null);
});

test('không có đích thì là sân chơi, không thắng thua', () => {
  const r = chayTrenThanhSo(py, 'di(3)\nnhan(-2)', { tu: -10, den: 10, bat_dau: 0 });
  assert.equal(r.thang, true);
  assert.equal(r.vi_sao, null);
});

test('mỗi lần chạy là một thanh số mới', () => {
  chay('di(6)');
  const r = chay('noi(dang_o())');
  assert.equal(r.su_kien.find((s) => s.kind === 'print').payload.cau, '0');
});
