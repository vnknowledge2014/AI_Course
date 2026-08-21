import { test, before } from 'node:test';
import assert from 'node:assert/strict';
import { chayTrenLuoi } from '../dist/the-gioi.js';

let py;
before(async () => {
  const { loadPyodide } = await import('pyodide');
  py = await loadPyodide();
}, { timeout: 120_000 });

const LUOI = {
  rong: 4,
  cao: 3,
  bat_dau: { x: 0, y: 0 },
  huong: 0,
  tuong: [{ x: 2, y: 0 }],
  vien: [{ x: 1, y: 0 }, { x: 3, y: 2 }],
};

test('đi thẳng thì Byte nhích đúng một ô mỗi lệnh', () => {
  const r = chayTrenLuoi(py, 'di_toi()', LUOI);
  const di = r.su_kien.filter((s) => s.kind === 'move');
  assert.equal(di.length, 1);
  assert.deepEqual(di[0].payload, { x: 1, y: 0 });
});

test('đi qua ô có viên thì nhặt được', () => {
  const r = chayTrenLuoi(py, 'di_toi()', LUOI);
  assert.equal(r.su_kien.filter((s) => s.kind === 'retrieve').length, 1);
});

/** Đây là lý do có sân khấu: sai một bước thì THẤY ngay, không phải đọc chữ. */
test('đâm tường thì dừng và nói ra bằng tiếng Việt', () => {
  const r = chayTrenLuoi(py, 'di_toi()\ndi_toi()', LUOI);
  assert.equal(r.thang, false);
  assert.match(r.vi_sao, /tường/);
  assert.ok(r.su_kien.some((s) => s.kind === 'panic'));
  // Bước hỏng KHÔNG được ghi là đã đi: người học phải thấy Byte dừng TRƯỚC bức
  // tường, không phải đứng trong nó.
  assert.ok(!r.su_kien.some((s) => s.kind === 'move' && s.payload.x === 2));
});

test('ra khỏi lưới cũng là đâm tường', () => {
  const r = chayTrenLuoi(py, 'quay_trai()\ndi_toi()', LUOI);
  assert.equal(r.thang, false);
  assert.match(r.vi_sao, /tường/);
});

test('nhặt hết viên mới thắng', () => {
  const chua_het = chayTrenLuoi(py, 'di_toi()', LUOI);
  assert.equal(chua_het.thang, false);
  assert.match(chua_het.vi_sao, /Còn 1 viên/);

  // 0,0 → 1,0 (viên) → xuống 1,1 → 1,2 → phải 2,2 → 3,2 (viên)
  const het = chayTrenLuoi(
    py,
    'di_toi()\nquay_phai()\ndi_toi(2)\nquay_trai()\ndi_toi(2)',
    LUOI,
  );
  assert.equal(het.thang, true, het.vi_sao ?? '');
  assert.equal(het.vi_sao, null);
});

test('câu hỏi về thế giới trả lời đúng', () => {
  // Quay lưng vào mép trái: phía trước không trống.
  const r = chayTrenLuoi(py, 'quay_trai()\nquay_trai()\nnoi(phia_truoc_trong())', LUOI);
  const noi = r.su_kien.find((s) => s.kind === 'print');
  assert.equal(noi.payload.cau, 'False');
});

test('vòng lặp điều khiển được Byte — đây là điểm nối với bài dạy `for`', () => {
  const r = chayTrenLuoi(py, 'for _ in range(1):\n    di_toi()', LUOI);
  assert.equal(r.su_kien.filter((s) => s.kind === 'move').length, 1);
});

test('mỗi lần chạy là một thế giới mới, không rớt trạng thái từ lần trước', () => {
  chayTrenLuoi(py, 'di_toi()', LUOI);
  const r = chayTrenLuoi(py, 'noi("lai tu dau")', LUOI);
  const dau = r.su_kien.find((s) => s.kind === 'alloc');
  assert.deepEqual(dau.payload, { x: 0, y: 0, huong: 0 });
  assert.equal(r.su_kien.filter((s) => s.kind === 'move').length, 0);
});
