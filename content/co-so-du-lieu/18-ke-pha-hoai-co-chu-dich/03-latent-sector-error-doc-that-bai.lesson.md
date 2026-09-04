---
id: co-so-du-lieu.ke-pha-hoai-co-chu-dich.latent-sector-error-doc-that-bai
title: "Latent sector error — đọc thất bại"
summary: "danhDauLoiSectorAn(sector) làm MỌI read() tiếp theo trên sector đó NÉM lỗi -- khác lost fsync/torn write (chỉ lộ SAU crash), lỗi này lộ NGAY, không cần crash. docCoPhucHoi bắt lỗi, ghi lại + fsync() (reallocate-on-rewrite) để đọc lại được. Xác nhận: đọc trực tiếp ném lỗi; sau khi ghi lại + fsync, đọc lại thành công VÀ vẫn thành công cả những lần đọc SAU đó nữa (lỗi đã bị xoá hẳn, không phải chỉ một lần)."
locale: vi
track: co-so-du-lieu
module: ke-pha-hoai-co-chu-dich
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.latent-sector-error-doc-that-bai]
requires: [db.chon-loi-bang-seed-khong-phai-nguoi]
concepts: [db.latent-sector-error-doc-that-bai]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Lost fsync VÀ torn write (R6-1, q18 bài 2) đều CHỈ lộ ra SAU một
`crash()`. Có lỗi đĩa nào lộ ra NGAY, không cần chờ mất điện không?
::::

::::explain{#latent-sector-error}
`danhDauLoiSectorAn(sector)` mô phỏng đúng lỗi đĩa thật gọi LÀ "latent
sector error" — đầu đọc VẬT LÝ không đọc được sector đó nữa. Không cần
`crash()` GÌ cả: `read()` NÉM lỗi ngay, mọi lần gọi, CHO TỚI KHI sector
được ghi LẠI và `fsync()` thành công (đĩa thật "reallocate" sector hỏng
khi ghi đè bền):

```typescript title=readonly
class SimDisk {
  readonly kichThuocSector: number;
  private readonly platter = new Map<number, Uint8Array>();
  private readonly cache = new Map<number, Uint8Array>();
  private readonly sectorLoiAn = new Set<number>();
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  read(sector: number): Uint8Array {
    if (this.sectorLoiAn.has(sector)) throw new Error(`latent sector error: sector ${sector}`);
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }
  write(sector: number, data: Uint8Array): void { this.cache.set(sector, data.slice()); }
  fsync(): void {
    for (const [sector, data] of this.cache) {
      this.platter.set(sector, data);
      this.sectorLoiAn.delete(sector); // ghi ben xoa loi -- "reallocate"
    }
    this.cache.clear();
  }
  danhDauLoiSectorAn(sector: number): void { this.sectorLoiAn.add(sector); }
}

function docCoPhucHoi(disk: SimDisk, sector: number, duLieuSua: Uint8Array): Uint8Array {
  try {
    return disk.read(sector);
  } catch {
    disk.write(sector, duLieuSua);
    disk.fsync();
    return disk.read(sector);
  }
}

const disk = new SimDisk(4);
disk.write(0, new Uint8Array([1, 2, 3, 4]));
disk.fsync();
disk.danhDauLoiSectorAn(0);

try {
  disk.read(0);
  console.log("khong nem loi -- SAI");
} catch (e) {
  console.log("doc truc tiep:", (e as Error).message);
}
console.log("phuc hoi:", [...docCoPhucHoi(disk, 0, new Uint8Array([9, 9, 9, 9]))]);
console.log("doc lai lan nua (khong qua catch):", [...disk.read(0)]);
```

```text title=readonly
doc truc tiep: latent sector error: sector 0
phuc hoi: [ 9, 9, 9, 9 ]
doc lai lan nua (khong qua catch): [ 9, 9, 9, 9 ]
```

Khác lost fsync/torn write — vốn LÀ "bẫy một lần", tự tắt sau ĐÚNG một
lần `fsync()` bị ảnh hưởng — latent sector error LÀ một TRẠNG THÁI gắn
với sector, tồn tại CHO TỚI KHI chính sector đó được ghi lại VÀ fsync
thành công. `docCoPhucHoi` bắt lỗi RỒI ghi đè + fsync để "reallocate":
sau đó MỌI lần đọc tiếp theo đều thành công, không chỉ lần đầu.
::::

::::example{#khong-giong-checksum}
Đây LÀ lý do các hệ thống lưu trữ thật (VÍ dụ ZFS, hay chính TigerBeetle)
luôn có một tầng "phục hồi khi đọc lỗi" — không thể chỉ dựa vào việc
"lần trước ghi đúng" (checksum) VÌ latent sector error KHÔNG làm sai dữ
liệu, nó làm sector KHÔNG đọc được GÌ cả — checksum không cứu được một
byte nào không lấy ra được.
::::

::::predict{#doan-doc-lai-lan-thu-ba commitOnce}
SAU đoạn code trên (đã `docCoPhucHoi` một lần, sector 0 đã "reallocate"),
gọi thêm `disk.danhDauLoiSectorAn(0)` MỘT lần NỮA rồi `disk.read(0)`
ngay. Kết quả LÀ gì?
:::opt{correct}
Ném lỗi lại — `danhDauLoiSectorAn` CÓ thể gọi lại bất kỳ lúc nào, mỗi
lần đánh dấu LÀ một lần lỗi MỚI, không liên quan gì tới việc sector đã
từng được phục hồi trước ĐÓ
:::
:::opt
Không ném lỗi — sector 0 đã "reallocate" một lần rồi thì MIỄN nhiễm
với latent sector error VĨNH viễn
::why
Trực giác NÀY nhầm "đã sửa một lần" VỚI "không thể hỏng lại" — nhưng
`sectorLoiAn` chỉ LÀ một `Set`, và `danhDauLoiSectorAn` chỉ ĐƠN giản
thêm sector vào ĐÓ mỗi khi được gọi.

Chỗ lệch: KHÔNG có "bộ nhớ" nào ghi lại rằng sector 0 "đã từng lỗi rồi
được sửa" — trạng thái CHỈ là "sector 0 CÓ đang trong `sectorLoiAn`
hay không" NGAY lúc này. Gọi `danhDauLoiSectorAn(0)` LẦN nữa đơn giản
thêm lại nó vào tập hợp đó, y hệt lần đầu.
::
:::
::::

::::code{#viet_doc_co_phuc_hoi}
Hoàn thiện `docCoPhucHoi` — khi `disk.read(sector)` ném lỗi, ghi lại
`duLieuSua`, `fsync()`, rồi đọc lại VÀ trả về kết quả đó.

```typescript title=starter
class SimDisk {
  readonly kichThuocSector: number;
  private readonly platter = new Map<number, Uint8Array>();
  private readonly cache = new Map<number, Uint8Array>();
  private readonly sectorLoiAn = new Set<number>();
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  read(sector: number): Uint8Array {
    if (this.sectorLoiAn.has(sector)) throw new Error(`latent sector error: sector ${sector}`);
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }
  write(sector: number, data: Uint8Array): void { this.cache.set(sector, data.slice()); }
  fsync(): void {
    for (const [sector, data] of this.cache) {
      this.platter.set(sector, data);
      this.sectorLoiAn.delete(sector);
    }
    this.cache.clear();
  }
  danhDauLoiSectorAn(sector: number): void { this.sectorLoiAn.add(sector); }
}

function docCoPhucHoi(disk: SimDisk, sector: number, duLieuSua: Uint8Array): Uint8Array {
  try {
    return disk.read(sector);
  } catch {
    ___
  }
}

const disk = new SimDisk(4);
disk.write(0, new Uint8Array([1, 2, 3, 4]));
disk.fsync();
disk.danhDauLoiSectorAn(0);
console.log([...docCoPhucHoi(disk, 0, new Uint8Array([9, 9, 9, 9]))]);
```

```typescript title=solution
class SimDisk {
  readonly kichThuocSector: number;
  private readonly platter = new Map<number, Uint8Array>();
  private readonly cache = new Map<number, Uint8Array>();
  private readonly sectorLoiAn = new Set<number>();
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  read(sector: number): Uint8Array {
    if (this.sectorLoiAn.has(sector)) throw new Error(`latent sector error: sector ${sector}`);
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }
  write(sector: number, data: Uint8Array): void { this.cache.set(sector, data.slice()); }
  fsync(): void {
    for (const [sector, data] of this.cache) {
      this.platter.set(sector, data);
      this.sectorLoiAn.delete(sector);
    }
    this.cache.clear();
  }
  danhDauLoiSectorAn(sector: number): void { this.sectorLoiAn.add(sector); }
}

function docCoPhucHoi(disk: SimDisk, sector: number, duLieuSua: Uint8Array): Uint8Array {
  try {
    return disk.read(sector);
  } catch {
    disk.write(sector, duLieuSua);
    disk.fsync();
    return disk.read(sector);
  }
}

const disk = new SimDisk(4);
disk.write(0, new Uint8Array([1, 2, 3, 4]));
disk.fsync();
disk.danhDauLoiSectorAn(0);
console.log([...docCoPhucHoi(disk, 0, new Uint8Array([9, 9, 9, 9]))]);
```

```typescript title=test
const diskT = new SimDisk(4);
diskT.write(0, new Uint8Array([1, 2, 3, 4]));
diskT.fsync();
diskT.write(1, new Uint8Array([5, 5, 5, 5]));
diskT.fsync();
diskT.danhDauLoiSectorAn(0);

let nemLoiKhiDocTrucTiep = false;
try { diskT.read(0); } catch { nemLoiKhiDocTrucTiep = true; }
if (!nemLoiKhiDocTrucTiep) throw new Error("doc truc tiep sector co latent error PHAI nem loi");

const ketQua = docCoPhucHoi(diskT, 0, new Uint8Array([9, 9, 9, 9]));
if (JSON.stringify([...ketQua]) !== "[9,9,9,9]") throw new Error("docCoPhucHoi phai tra ve DUNG du lieu moi da ghi lai, [9,9,9,9]");

const docLaiKhongQuaCatch = diskT.read(0);
if (JSON.stringify([...docLaiKhongQuaCatch]) !== "[9,9,9,9]") throw new Error("sau phuc hoi, doc TRUC TIEP (khong qua try/catch) van phai thanh cong -- loi da bi xoa han (reallocate)");

if (JSON.stringify([...diskT.read(1)]) !== "[5,5,5,5]") throw new Error("sector khac (1) khong bi anh huong boi loi cua sector 0");

const ketQuaKhongLoi = docCoPhucHoi(diskT, 1, new Uint8Array([0, 0, 0, 0]));
if (JSON.stringify([...ketQuaKhongLoi]) !== "[5,5,5,5]") throw new Error("docCoPhucHoi tren sector KHONG loi phai tra ve dung du lieu HIEN CO, khong duoc ghi de duLieuSua");
```

:::hints
- kind: attention
  body: "Trong catch: write(sector, duLieuSua), fsync(), roi return read(sector) -- ba dong."
- kind: strategy
  body: "disk.write(sector, duLieuSua); disk.fsync(); return disk.read(sector);"
- kind: one-line
  body: "disk.write(sector, duLieuSua); disk.fsync(); return disk.read(sector);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "9,9,9,9"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đọc sai vì SECTOR hỏng. Lỗi tiếp theo tệ hơn: đọc ĐÚNG dữ liệu, checksum
ĐÚNG, chỉ SAI vị trí — vì đầu ghi lạc đường.
::::

::::reflect{#nghi-lai}
`docCoPhucHoi` LÀ mẫu hình chung của MỌI "phục hồi khi đọc lỗi" trong
hệ thống lưu trữ thật: bắt lỗi, ghi LẠI (không cố "vá" phần còn đọc
được — dữ liệu VẬT LÝ đã mất), fsync để bền, rồi thử LẠI. Khác biệt
lớn nhất SO với lost fsync/torn write: đây LÀ lỗi lộ NGAY khi đọc,
không cần đợi `crash()` mới thấy.
::::

::::checkpoint{mastery=0.85}
::::
