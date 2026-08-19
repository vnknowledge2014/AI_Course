import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { BoThucThiRust } from '../dist/index.js';

const WASM = new URL('../../byte-rust/target/wasm32-unknown-unknown/release/byte_rust.wasm', import.meta.url);
const engine = new BoThucThiRust(() => readFile(WASM));

test('chạy chương trình đúng', async () => {
  const r = await engine.chay('fn main() { println!("chào"); }');
  assert.equal(r.ok, true);
  assert.equal(r.xuat, 'chào\n');
  assert.equal(r.chanDoan.length, 0);
  assert.equal(r.biNgat, false);
});

test('chẩn đoán được đổi sang camelCase đầy đủ', async () => {
  const r = await engine.chay('fn main() { let x = 1 / 0; }');
  assert.equal(r.ok, false);
  const d = r.chanDoan[0];
  assert.equal(d.ma, 'BR0532');
  assert.equal(d.muc, 'loi');
  assert.ok(d.thongDiep.length > 0);
  assert.equal(typeof d.dong, 'number');
  assert.equal(typeof d.cot, 'number');
  assert.ok(d.viSao, 'phải có tầng giải thích');
  assert.ok(Array.isArray(d.cachSua) && d.cachSua.length > 0, 'phải có cách sửa');
  assert.ok(d.vanBan.includes('-->'), 'phải có bản kết xuất có toạ độ');
});

test('vòng lặp vô hạn được đánh dấu biNgat, không treo', async () => {
  const r = await engine.chay('fn main() { while true {} }');
  assert.equal(r.ok, false);
  assert.equal(r.biNgat, true, 'phải nhận ra là bị ngắt chứ không phải lỗi thường');
  assert.ok(r.thoiGianMs < 5000, `chạy quá lâu: ${r.thoiGianMs}ms`);
});

test('maKiemTra được nối vào sau mã người học', async () => {
  const r = await engine.chay('fn cong(a: i64, b: i64) -> i64 { a + b }', {
    maKiemTra: 'fn main() { assert_eq!(cong(2, 3), 5); println!("dat"); }',
  });
  assert.equal(r.ok, true, r.chanDoan[0]?.vanBan);
  assert.equal(r.xuat, 'dat\n');
});

test('maKiemTra phát hiện được lời giải SAI', async () => {
  const r = await engine.chay('fn cong(a: i64, b: i64) -> i64 { a - b }', {
    maKiemTra: 'fn main() { assert_eq!(cong(2, 3), 5); }',
  });
  assert.equal(r.ok, false, 'lời giải sai KHÔNG được báo pass');
  assert.equal(r.chanDoan[0].ma, 'BR0541');
});

test('tiếng Việt đi qua ranh giới WASM nguyên vẹn', async () => {
  const r = await engine.chay('fn main() { println!("Nguyễn Văn A — 25 tuổi"); }');
  assert.equal(r.xuat, 'Nguyễn Văn A — 25 tuổi\n');
});

test('gọi liên tiếp 200 lần không rò rỉ và không hỏng trạng thái', async () => {
  for (let i = 0; i < 200; i++) {
    const r = await engine.chay(`fn main() { println!("{}", ${i} * 2); }`);
    assert.equal(r.xuat, `${i * 2}\n`);
  }
});

test('mã lỗi cú pháp cũng trả về đúng dạng', async () => {
  const r = await engine.chay('fn main() { let x = ; }');
  assert.equal(r.ok, false);
  assert.ok(r.chanDoan.length > 0);
  assert.ok(r.chanDoan[0].dong >= 1);
});
