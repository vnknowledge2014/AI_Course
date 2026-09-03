import { test } from 'node:test';
import assert from 'node:assert/strict';
import { SimDisk } from '../dist/index.js';

test('sector chưa ghi đọc ra toàn số 0', () => {
  const d = new SimDisk({ soLuongSector: 4, kichThuocSector: 8 });
  assert.deepEqual(d.read(0), new Uint8Array(8));
});

test('write rồi read (chưa fsync) vẫn thấy dữ liệu mới', () => {
  const d = new SimDisk({ soLuongSector: 4, kichThuocSector: 4 });
  d.write(1, new Uint8Array([1, 2, 3, 4]));
  assert.deepEqual(d.read(1), new Uint8Array([1, 2, 3, 4]));
});

test('crash TRƯỚC fsync xoá mất ghi', () => {
  const d = new SimDisk({ soLuongSector: 4, kichThuocSector: 4 });
  d.write(1, new Uint8Array([1, 2, 3, 4]));
  d.crash();
  assert.deepEqual(d.read(1), new Uint8Array(4));
});

test('crash SAU fsync giữ nguyên ghi', () => {
  const d = new SimDisk({ soLuongSector: 4, kichThuocSector: 4 });
  d.write(1, new Uint8Array([1, 2, 3, 4]));
  d.fsync();
  d.crash();
  assert.deepEqual(d.read(1), new Uint8Array([1, 2, 3, 4]));
});

test('coGhiChuaFsync phản ánh đúng trạng thái cache', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 2 });
  assert.equal(d.coGhiChuaFsync(), false);
  d.write(0, new Uint8Array([9, 9]));
  assert.equal(d.coGhiChuaFsync(), true);
  d.fsync();
  assert.equal(d.coGhiChuaFsync(), false);
});

test('sector ngoài phạm vi ném RangeError', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 2 });
  assert.throws(() => d.read(2), RangeError);
  assert.throws(() => d.read(-1), RangeError);
  assert.throws(() => d.write(2, new Uint8Array([0, 0])), RangeError);
});

test('write sai kích thước ném RangeError', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 4 });
  assert.throws(() => d.write(0, new Uint8Array([1, 2])), RangeError);
});

test('read trả về bản sao, không phải tham chiếu sống vào cache/platter', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 2 });
  d.write(0, new Uint8Array([1, 2]));
  const a = d.read(0);
  a[0] = 99;
  assert.deepEqual(d.read(0), new Uint8Array([1, 2]));
});

test('nhiều sector độc lập nhau', () => {
  const d = new SimDisk({ soLuongSector: 3, kichThuocSector: 2 });
  d.write(0, new Uint8Array([1, 1]));
  d.write(2, new Uint8Array([2, 2]));
  d.fsync();
  assert.deepEqual(d.read(0), new Uint8Array([1, 1]));
  assert.deepEqual(d.read(1), new Uint8Array([0, 0]));
  assert.deepEqual(d.read(2), new Uint8Array([2, 2]));
});
