import { test, before } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { BoThucThiRust } from '../dist/index.js';

/** Chạy `byte_rust.wasm` THẬT, qua đúng đường mà ứng dụng dùng.
 *
 *  Các test khác của crate chạy interpreter ở tầng Rust. Test này kiểm tầng
 *  còn lại: module WASM có nạp được không, ABI trần (`br_cap_phat` /
 *  `br_chay` / `br_giai_phong`) có khớp không, và chuỗi UTF-8 tiếng Việt đi
 *  qua ranh giới ấy có nguyên vẹn không. Ba thứ đó hỏng riêng được, và hỏng
 *  thì không test Rust nào thấy.
 */
let may;

before(async () => {
  const wasm = new URL(
    '../../../target/wasm32-unknown-unknown/release/byte_rust.wasm',
    import.meta.url,
  );
  may = new BoThucThiRust(async () => readFile(wasm));
  await may.sanSang();
}, { timeout: 60_000 });

test('chạy được chương trình Rust hợp lệ', async () => {
  const r = await may.chay('fn main() {\n    println!("Xin chào");\n}');
  assert.equal(r.ok, true, JSON.stringify(r.chanDoan));
  assert.match(r.xuat, /Xin chào/);
});

test('chuỗi tiếng Việt qua ranh giới WASM vẫn nguyên', async () => {
  // ABI là byte trần, nên dấu tiếng Việt là chỗ đầu tiên hỏng nếu ai đó
  // nhầm độ dài chuỗi với số ký tự.
  const r = await may.chay('fn main() {\n    println!("Phở bò tái nạm — 45.000đ");\n}');
  assert.equal(r.ok, true);
  assert.match(r.xuat, /Phở bò tái nạm — 45\.000đ/);
});

test('từ chối mã sai kiểu, và nói ra mã lỗi', async () => {
  const r = await may.chay('fn main() {\n    let n: i64 = "ba";\n}');
  assert.equal(r.ok, false);
  assert.ok(r.chanDoan.length > 0, 'phải có chẩn đoán, không chỉ ok=false');
  assert.ok(r.chanDoan[0].ma.startsWith('BR'), `mã lạ: ${r.chanDoan[0].ma}`);
});

test('chẩn đoán có đủ ba tầng', async () => {
  const r = await may.chay('fn main() {\n    let x: u8 = 300;\n}');
  const c = r.chanDoan[0];
  assert.ok(c, 'không có chẩn đoán nào');
  assert.ok(c.thongDiep, 'thiếu tầng 1 — chuyện gì xảy ra');
  assert.ok(c.viSao, 'thiếu tầng 2 — vì sao Rust không cho phép');
  assert.ok(c.cachSua.length > 0, 'thiếu tầng 3 — sửa thế nào');
});

test('vòng lặp vô hạn bị chặn ở tầng Rust, không treo host', async () => {
  // Đây là lý do ADR-001 chọn viết interpreter thay vì mượn runtime: ngân
  // sách nhiên liệu nằm BÊN TRONG, nên không cần Worker để giết ai cả.
  const t0 = Date.now();
  const r = await may.chay('fn main() {\n    loop {}\n}');
  assert.ok(Date.now() - t0 < 20_000, 'chạy quá lâu — ngân sách nhiên liệu không hoạt động');
  assert.equal(r.ok, false);
});
