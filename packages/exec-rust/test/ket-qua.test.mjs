import { test } from 'node:test';
import assert from 'node:assert/strict';
import { ketQuaTuThoRust } from '../dist/index.js';

// Test THUẦN — không nạp WASM. Hàm dưới test là khúc diễn dịch dùng chung của
// cả hai host (WASM trong trình duyệt và command `chay_rust` gọi qua Tauri
// native), nên khúc này trệch thì CẢ HAI nền tảng trệch theo — không có host
// nào "đúng hơn" để so ra. Hợp đồng với phía Rust là chuỗi JSON từ
// `chay_thanh_json`, đã có test trong crates/byte-rust/src/wasm.rs.

function chanDoanTho(ma, qua = {}) {
  return {
    ma,
    muc: 'loi',
    thong_diep: 'thong diep',
    dong: 3,
    cot: 5,
    do_dai: 7,
    vi_sao: 'vi sao',
    cach_sua: ['sua the nay'],
    khai_niem: 'khai niem',
    van_ban: 'van ban',
    ...qua,
  };
}

test('ketQuaTuThoRust: chương trình đúng — ok, xuất trần, không chẩn đoán', () => {
  const r = ketQuaTuThoRust({ ok: true, xuat: 'chào\n', chan_doan: [] }, 12.5);
  assert.equal(r.ok, true);
  assert.equal(r.xuat, 'chào\n');
  assert.deepEqual(r.chanDoan, []);
  assert.equal(r.thoiGianMs, 12.5);
  assert.equal(r.biNgat, false);
});

test('ketQuaTuThoRust: BR0500 (hết nhiên liệu) => biNgat', () => {
  const r = ketQuaTuThoRust({ ok: false, xuat: '', chan_doan: [chanDoanTho('BR0500')] }, 1);
  assert.equal(r.biNgat, true);
});

test('ketQuaTuThoRust: BR0505 (đệ quy quá sâu) => biNgat', () => {
  const r = ketQuaTuThoRust({ ok: false, xuat: '', chan_doan: [chanDoanTho('BR0505')] }, 1);
  assert.equal(r.biNgat, true);
});

test('ketQuaTuThoRust: lỗi thường KHÔNG phải ngắt — chỉ hai mã trên mới là ngắt', () => {
  const r = ketQuaTuThoRust({ ok: false, xuat: '', chan_doan: [chanDoanTho('BR0532')] }, 1);
  assert.equal(r.biNgat, false);
});

test('ketQuaTuThoRust: chẩn đoán đổi snake_case sang camelCase đầy đủ', () => {
  const r = ketQuaTuThoRust({ ok: false, xuat: '', chan_doan: [chanDoanTho('BR0532')] }, 1);
  const d = r.chanDoan[0];
  assert.equal(d.ma, 'BR0532');
  assert.equal(d.muc, 'loi');
  assert.equal(d.thongDiep, 'thong diep');
  assert.equal(d.dong, 3);
  assert.equal(d.cot, 5);
  assert.equal(d.doDai, 7);
  assert.equal(d.viSao, 'vi sao');
  assert.deepEqual(d.cachSua, ['sua the nay']);
  assert.equal(d.khaiNiem, 'khai niem');
  assert.equal(d.vanBan, 'van ban');
});

test('ketQuaTuThoRust: null lan truyền nguyên trạng (vi_sao/khai_niem)', () => {
  const r = ketQuaTuThoRust(
    { ok: false, xuat: '', chan_doan: [chanDoanTho('BR9999', { vi_sao: null, khai_niem: null })] },
    1,
  );
  assert.equal(r.chanDoan[0].viSao, null);
  assert.equal(r.chanDoan[0].khaiNiem, null);
});
