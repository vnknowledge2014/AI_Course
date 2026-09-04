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

test('boQuaFsyncKeTiep -- fsync ke tiep la no-op, read() van dung, crash() moi lo', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 2 });
  d.write(0, new Uint8Array([9, 9]));
  d.boQuaFsyncKeTiep();
  d.fsync();
  assert.deepEqual(d.read(0), new Uint8Array([9, 9]), 'read van thay du lieu (con trong cache)');
  assert.equal(d.coGhiChuaFsync(), true, 'cache CHUA thuc su duoc day xuong platter');
  d.crash();
  assert.deepEqual(d.read(0), new Uint8Array([0, 0]), 'crash loi ra: du lieu mat that');
});

test('boQuaFsyncKeTiep chi anh huong DUNG mot lan fsync', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 2 });
  d.write(0, new Uint8Array([9, 9]));
  d.boQuaFsyncKeTiep();
  d.fsync();
  d.fsync();
  d.crash();
  assert.deepEqual(d.read(0), new Uint8Array([9, 9]), 'lan fsync thu hai da flush that');
});

test('danhDauTornGhi -- chi ghi thanh cong N byte dau, phan con lai giu du lieu cu', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 4 });
  d.write(0, new Uint8Array([1, 1, 1, 1]));
  d.fsync();
  d.write(0, new Uint8Array([2, 2, 2, 2]));
  d.danhDauTornGhi(0, 2);
  d.fsync();
  assert.deepEqual(d.read(0), new Uint8Array([2, 2, 1, 1]), 'hai byte dau moi, hai byte sau van cu');
});

test('danhDauTornGhi tren sector chua tung ghi -- phan con lai la so 0', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 4 });
  d.write(0, new Uint8Array([9, 9, 9, 9]));
  d.danhDauTornGhi(0, 1);
  d.fsync();
  assert.deepEqual(d.read(0), new Uint8Array([9, 0, 0, 0]));
});

test('danhDauTornGhi chi anh huong DUNG mot lan fsync tiep theo', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 2 });
  d.write(0, new Uint8Array([1, 1]));
  d.danhDauTornGhi(0, 0);
  d.fsync();
  assert.deepEqual(d.read(0), new Uint8Array([0, 0]), 'lan dau: torn, 0 byte thanh cong');
  d.write(0, new Uint8Array([5, 5]));
  d.fsync();
  assert.deepEqual(d.read(0), new Uint8Array([5, 5]), 'lan hai: fsync binh thuong');
});

test('danhDauTornGhi bao loi khi soByteThanhCong ngoai pham vi', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 4 });
  assert.throws(() => d.danhDauTornGhi(0, 5), RangeError);
  assert.throws(() => d.danhDauTornGhi(0, -1), RangeError);
});

test('danhDauLoiSectorAn -- read() nem loi, KHONG anh huong sector khac', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 2 });
  d.write(0, new Uint8Array([1, 1]));
  d.write(1, new Uint8Array([2, 2]));
  d.fsync();
  d.danhDauLoiSectorAn(0);
  assert.throws(() => d.read(0), Error);
  assert.deepEqual(d.read(1), new Uint8Array([2, 2]), 'sector khac khong bi anh huong');
});

test('danhDauLoiSectorAn -- ghi lai roi fsync() thi read() lai duoc (reallocate)', () => {
  const d = new SimDisk({ soLuongSector: 1, kichThuocSector: 2 });
  d.write(0, new Uint8Array([1, 1]));
  d.fsync();
  d.danhDauLoiSectorAn(0);
  assert.throws(() => d.read(0), Error);
  d.write(0, new Uint8Array([9, 9]));
  d.fsync();
  assert.deepEqual(d.read(0), new Uint8Array([9, 9]), 'ghi lai + fsync xoa loi, doc duoc gia tri MOI');
});

test('danhDauLoiSectorAn -- chi write() (chua fsync) KHONG xoa loi', () => {
  const d = new SimDisk({ soLuongSector: 1, kichThuocSector: 2 });
  d.write(0, new Uint8Array([1, 1]));
  d.fsync();
  d.danhDauLoiSectorAn(0);
  d.write(0, new Uint8Array([9, 9]));
  assert.throws(() => d.read(0), Error, 'chua fsync thi loi van con, du da write() lai');
});

test('danhDauGhiSaiDich -- du lieu ghi nham sang sector khac, sector dinh KHONG doi', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 2 });
  d.write(0, new Uint8Array([5, 5]));
  d.fsync();
  d.write(1, new Uint8Array([7, 7]));
  d.danhDauGhiSaiDich(1, 0);
  d.fsync();
  assert.deepEqual(d.read(0), new Uint8Array([7, 7]), 'sector THUC (0) bi ghi de nham');
  assert.deepEqual(d.read(1), new Uint8Array([0, 0]), 'sector DICH (1) chua tung duoc ghi that, van la 0');
});

test('danhDauGhiSaiDich chi anh huong DUNG mot lan fsync tiep theo', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 2 });
  d.danhDauGhiSaiDich(1, 0);
  d.write(1, new Uint8Array([7, 7]));
  d.fsync();
  assert.deepEqual(d.read(0), new Uint8Array([7, 7]));
  d.write(1, new Uint8Array([8, 8]));
  d.fsync();
  assert.deepEqual(d.read(1), new Uint8Array([8, 8]), 'lan hai fsync binh thuong, dung dich');
  assert.deepEqual(d.read(0), new Uint8Array([7, 7]), 'sector 0 khong doi them lan nua');
});

test('datCrashSauThaoTacThuN -- crash tu dong dung sau N lan write()', () => {
  const d = new SimDisk({ soLuongSector: 3, kichThuocSector: 2 });
  d.datCrashSauThaoTacThuN(3);
  d.write(0, new Uint8Array([1, 1]));
  d.write(1, new Uint8Array([2, 2]));
  assert.equal(d.coGhiChuaFsync(), true, 'chua den lan ghi thu 3, cache van con');
  d.write(2, new Uint8Array([3, 3]));
  assert.equal(d.coGhiChuaFsync(), false, 'crash() da tu dong chay dung sau lan write() thu 3');
  assert.deepEqual(d.read(0), new Uint8Array(2), 'du lieu chua fsync bi mat that, dung la crash that');
});

test('datCrashSauThaoTacThuN -- dem tu THOI DIEM goi ham, khong tinh write() truoc do', () => {
  const d = new SimDisk({ soLuongSector: 2, kichThuocSector: 2 });
  d.write(0, new Uint8Array([1, 1]));
  d.write(1, new Uint8Array([2, 2]));
  d.datCrashSauThaoTacThuN(1);
  assert.equal(d.coGhiChuaFsync(), true, 'hai lan write TRUOC KHI dat lich khong bi tinh');
  d.write(0, new Uint8Array([9, 9]));
  assert.equal(d.coGhiChuaFsync(), false, 'dung lan write dau tien SAU khi dat lich thi crash');
});
