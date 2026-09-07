import { test } from 'node:test';
import assert from 'node:assert/strict';
import { chanDoanQuaGio } from '../dist/worker.js';

test('chanDoanQuaGio: mã và mức luôn cố định, đúng hợp đồng ChanDoan', () => {
  const c = chanDoanQuaGio(5000);
  assert.equal(c.ma, 'EX0500');
  assert.equal(c.muc, 'loi');
  assert.equal(c.dong, 1);
  assert.equal(c.cot, 1);
  assert.equal(c.doDai, 1);
  assert.equal(c.khaiNiem, 'vòng lặp');
});

test('chanDoanQuaGio: đổi mili-giây sang giây đúng, không làm tròn sai', () => {
  const c = chanDoanQuaGio(5000);
  assert.ok(c.thongDiep.includes('5 giây'), `thongDiep phải nêu đúng 5 giây, thực tế: ${c.thongDiep}`);
  assert.ok(c.vanBan.includes('5 giây'), `vanBan phải nêu đúng 5 giây, thực tế: ${c.vanBan}`);

  const c2 = chanDoanQuaGio(15000);
  assert.ok(c2.thongDiep.includes('15 giây'), `hetHanMs=15000 phải in ra 15 giây, thực tế: ${c2.thongDiep}`);

  // Ngân sách không tròn giây (vd 1500ms) vẫn phải quy đổi đúng, không làm
  // tròn thành số nguyên — sai một chữ số ở đây là lộ ra một ngưỡng KHÁC với
  // ngưỡng engine thật sự dùng, khiến người học không hiểu vì sao bị dừng.
  const c3 = chanDoanQuaGio(1500);
  assert.ok(c3.thongDiep.includes('1.5 giây'), `hetHanMs=1500 phải in ra 1.5 giây, thực tế: ${c3.thongDiep}`);
});

test('chanDoanQuaGio: luôn có ít nhất một cách sửa cụ thể, không rỗng', () => {
  const c = chanDoanQuaGio(8000);
  assert.ok(Array.isArray(c.cachSua) && c.cachSua.length > 0, 'cachSua không được rỗng — đây là tầng 3 của chẩn đoán');
  assert.ok(typeof c.viSao === 'string' && c.viSao.length > 0, 'viSao không được rỗng — đây là tầng 2 của chẩn đoán');
});
