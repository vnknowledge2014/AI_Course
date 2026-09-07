import { test } from 'node:test';
import assert from 'node:assert/strict';
import { doiChanDoanRust } from '../dist/types.js';

// doiChanDoanRust đổi JSON snake_case mà crates/byte-rust trả về sang
// camelCase mà cả app dùng. Một trường bị đổi sai tên hoặc bị bỏ sót ở đây
// KHÔNG gây lỗi biên dịch (cả hai phía đều là object thường) — nó chỉ lặng
// lẽ làm một trường của MỌI chẩn đoán Rust thành `undefined`, và học viên
// thấy thông báo lỗi cụt mất một tầng mà không ai biết vì sao.

const THO_DAY_DU = {
  ma: 'BR0530',
  muc: 'loi',
  thong_diep: 'kiểu không khớp',
  dong: 7,
  cot: 3,
  do_dai: 4,
  vi_sao: 'x là số nguyên, không thể cộng với chuỗi',
  cach_sua: ['đổi x sang chuỗi bằng to_string()', 'đổi vế còn lại sang số'],
  khai_niem: 'ép kiểu',
  van_ban: 'lỗi [BR0530]: kiểu không khớp\n  --> dòng 7',
};

test('doiChanDoanRust: đổi ĐÚNG cả 10 trường, không sót không lệch tên', () => {
  const c = doiChanDoanRust(THO_DAY_DU);
  assert.equal(c.ma, THO_DAY_DU.ma);
  assert.equal(c.muc, THO_DAY_DU.muc);
  assert.equal(c.thongDiep, THO_DAY_DU.thong_diep);
  assert.equal(c.dong, THO_DAY_DU.dong);
  assert.equal(c.cot, THO_DAY_DU.cot);
  assert.equal(c.doDai, THO_DAY_DU.do_dai);
  assert.equal(c.viSao, THO_DAY_DU.vi_sao);
  assert.deepEqual(c.cachSua, THO_DAY_DU.cach_sua);
  assert.equal(c.khaiNiem, THO_DAY_DU.khai_niem);
  assert.equal(c.vanBan, THO_DAY_DU.van_ban);
});

test('doiChanDoanRust: cachSua là mảng MỚI đúng thứ tự, không tham chiếu ngược', () => {
  const c = doiChanDoanRust(THO_DAY_DU);
  assert.equal(c.cachSua.length, 2);
  assert.equal(c.cachSua[0], THO_DAY_DU.cach_sua[0]);
  assert.equal(c.cachSua[1], THO_DAY_DU.cach_sua[1]);
});

test('doiChanDoanRust: vi_sao/khai_niem null đi qua nguyên vẹn, không đổi thành chuỗi rỗng', () => {
  const tho = { ...THO_DAY_DU, vi_sao: null, khai_niem: null };
  const c = doiChanDoanRust(tho);
  assert.equal(c.viSao, null);
  assert.equal(c.khaiNiem, null);
});

test('doiChanDoanRust: muc canh_bao đi qua nguyên vẹn (không chỉ có loi)', () => {
  const tho = { ...THO_DAY_DU, muc: 'canh_bao' };
  const c = doiChanDoanRust(tho);
  assert.equal(c.muc, 'canh_bao');
});
