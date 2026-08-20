import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { createRequire } from 'node:module';
import TS from 'typescript';
import { kiemKieu } from '../dist/kiem-kieu.js';

const require = createRequire(import.meta.url);
const LIB = dirname(require.resolve('typescript'));
const doc_lib = (ten) => {
  try {
    return readFileSync(join(LIB, ten), 'utf-8');
  } catch {
    return undefined;
  }
};

const kiem = (ma) => kiemKieu(TS, ma, doc_lib);

test('mã đúng thì sạch và sinh được JavaScript', () => {
  const r = kiem('const ten: string = "Byte";\nconsole.log(ten.toUpperCase());');
  assert.deepEqual(r.chanDoan.filter((c) => c.muc === 'loi'), []);
  assert.ok(r.js && r.js.includes('toUpperCase'));
});

/** Đây là lý do tồn tại của cả file này.
 *
 *  Sucrase chỉ BÓC chú thích kiểu rồi chạy — nó không kiểm gì. Với nó, đoạn
 *  dưới chạy trót lọt và bài học báo ĐẠT, dù toàn bộ lý do người ta học
 *  TypeScript là hệ thống kiểu. Cùng một chế độ hỏng đã làm engine Rust giả
 *  trước đây vô dụng.
 */
test('gán sai kiểu bị TỪ CHỐI, và không sinh JavaScript', () => {
  const r = kiem('let n: number = "ba";');
  const loi = r.chanDoan.filter((c) => c.muc === 'loi');
  assert.equal(loi.length, 1);
  assert.equal(loi[0].ma, 'TS2322');
  assert.equal(r.js, null, 'mã đã biết chắc là sai thì KHÔNG được chạy');
  assert.ok(loi[0].viSao, 'phải có tầng "vì sao", không chỉ nhắc lại thông báo gốc');
});

test('đối số sai kiểu bị bắt', () => {
  const r = kiem('function gapDoi(x: number): number { return x * 2 }\ngapDoi("ba");');
  assert.ok(r.chanDoan.some((c) => c.ma === 'TS2345'));
  assert.equal(r.js, null);
});

test('strict bật: tham số ngầm any bị bắt', () => {
  const r = kiem('function f(x) { return x }');
  assert.ok(r.chanDoan.some((c) => c.ma === 'TS7006'));
});

test('noUncheckedIndexedAccess bật: v[0] có thể undefined', () => {
  // Nếu tắt luật này thì mọi bài học về null/undefined — phần khó nhất — sẽ
  // không bao giờ bắt được lỗi nào.
  const r = kiem('const v: number[] = [1, 2];\nconst a: number = v[0];');
  assert.ok(r.chanDoan.some((c) => c.muc === 'loi'), 'phải bắt được v[0] có thể undefined');
});

test('tên không tồn tại bị bắt dù nằm trong nhánh không chạy tới', () => {
  const r = kiem('if (false) { chuaHeViet(1); }');
  assert.ok(r.chanDoan.some((c) => c.ma === 'TS2304'));
  assert.equal(r.js, null);
});

test('vị trí lỗi trỏ đúng dòng', () => {
  const r = kiem('const a = 1;\nconst b = 2;\nlet n: number = "ba";');
  const loi = r.chanDoan.find((c) => c.ma === 'TS2322');
  assert.equal(loi.dong, 3);
});

test('engine TỪ CHỐI chạy mã sai kiểu, không gọi tới worker', async () => {
  const { BoThucThiTypeScript } = await import('../dist/index.js');
  let da_goi_worker = false;
  const may = new BoThucThiTypeScript(
    () => {
      da_goi_worker = true;
      return { gui: () => {}, khiNhan: () => {}, giet: () => {} };
    },
    (ma) => kiem(ma),
  );
  const r = await may.chay('let n: number = "ba";');
  assert.equal(r.ok, false);
  assert.equal(da_goi_worker, false, 'mã đã biết chắc sai thì không được đưa xuống worker');
  assert.ok(r.chanDoan.some((c) => c.ma === 'TS2322'));
});
