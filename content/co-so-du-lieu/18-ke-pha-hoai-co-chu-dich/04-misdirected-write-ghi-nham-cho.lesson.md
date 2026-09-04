---
id: co-so-du-lieu.ke-pha-hoai-co-chu-dich.misdirected-write-ghi-nham-cho
title: "Misdirected write — ghi nhầm chỗ"
summary: "danhDauGhiSaiDich(dinh,thuc) làm lần fsync() tiếp theo ghi dữ liệu ĐỊNH cho sectorDinhGhi lại vào sectorThucTe -- dữ liệu VÀ checksum-của-nó đều đúng, chỉ SAI vị trí. ghiCoNhan/docCoKiemTraViTri dùng bản ghi TỰ NHẬN DIỆN (byte đầu = sector định ghi) để phát hiện: sau khi ghi sector 2 (đánh dấu redirect sang sector 1), sector 1 đọc ra tag='2' -- SAI vị trí -- checksum-trên-từng-sector không hề bắt được lớp lỗi này."
locale: vi
track: co-so-du-lieu
module: ke-pha-hoai-co-chu-dich
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [db.misdirected-write-ghi-nham-cho]
requires: [db.latent-sector-error-doc-that-bai]
concepts: [db.misdirected-write-ghi-nham-cho]
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
Latent sector error làm ĐỌC thất bại rõ ràng — dễ phát hiện. Có lỗi
đĩa nào mà dữ liệu đọc RA vẫn "trông đúng hoàn toàn" nhưng lại SAI
không?
::::

::::explain{#misdirected-write}
`danhDauGhiSaiDich(sectorDinhGhi, sectorThucTe)` mô phỏng lỗi firmware:
đầu ghi SEEK sai sector. Lần `fsync()` tiếp theo, dữ liệu ĐỊNH cho
`sectorDinhGhi` lại ghi vào `sectorThucTe` — `sectorDinhGhi` giữ
NGUYÊN dữ liệu cũ, `sectorThucTe` bị ghi ĐÈ nhầm. Nguy hiểm hơn torn
write: dữ liệu (và checksum của NÓ, nếu có) đều "đúng" — chỉ sai VỊ
TRÍ. Cách duy nhất phát hiện: bản ghi phải TỰ NHẬN DIỆN — nhúng SẴN
"tôi thuộc về sector nào" NGAY trong payload:

```typescript title=readonly
class SimDisk {
  readonly kichThuocSector: number;
  private readonly platter = new Map<number, Uint8Array>();
  private readonly cache = new Map<number, Uint8Array>();
  private readonly sectorLoiAn = new Set<number>();
  private ghiSaiDichKeTiep: { dinh: number; thuc: number } | null = null;
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  read(sector: number): Uint8Array {
    if (this.sectorLoiAn.has(sector)) throw new Error(`latent sector error: sector ${sector}`);
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }
  write(sector: number, data: Uint8Array): void { this.cache.set(sector, data.slice()); }
  fsync(): void {
    const saiDich = this.ghiSaiDichKeTiep;
    this.ghiSaiDichKeTiep = null;
    for (const [sector, data] of this.cache) {
      if (saiDich !== null && sector === saiDich.dinh) {
        this.platter.set(saiDich.thuc, data);
        this.sectorLoiAn.delete(saiDich.thuc);
        continue;
      }
      this.platter.set(sector, data);
      this.sectorLoiAn.delete(sector);
    }
    this.cache.clear();
  }
  danhDauGhiSaiDich(sectorDinhGhi: number, sectorThucTe: number): void {
    this.ghiSaiDichKeTiep = { dinh: sectorDinhGhi, thuc: sectorThucTe };
  }
}

// Ban ghi TU NHAN DIEN: byte dau la sector DINH ghi.
function ghiCoNhan(disk: SimDisk, sector: number, payload: number[]): void {
  const buf = new Uint8Array(disk.kichThuocSector);
  buf[0] = sector;
  buf.set(payload, 1);
  disk.write(sector, buf);
}
function docCoKiemTraViTri(disk: SimDisk, sector: number): { duLieu: number[]; dungViTri: boolean } {
  const bytes = disk.read(sector);
  return { duLieu: [...bytes.slice(1)], dungViTri: bytes[0] === sector };
}

const disk = new SimDisk(4);
ghiCoNhan(disk, 1, [7, 7, 7]);
disk.fsync();
console.log("sector 1 truoc:", docCoKiemTraViTri(disk, 1));

disk.danhDauGhiSaiDich(2, 1); // lan fsync toi: du lieu dinh cho sector 2 lai ghi vao sector 1
ghiCoNhan(disk, 2, [8, 8, 8]);
disk.fsync();

console.log("sector 2 (dinh ghi, KHONG doi):", docCoKiemTraViTri(disk, 2));
console.log("sector 1 (thuc te bi ghi nham):", docCoKiemTraViTri(disk, 1));
```

```text title=readonly
sector 1 truoc: { duLieu: [ 7, 7, 7 ], dungViTri: true }
sector 2 (dinh ghi, KHONG doi): { duLieu: [ 0, 0, 0 ], dungViTri: false }
sector 1 (thuc te bi ghi nham): { duLieu: [ 8, 8, 8 ], dungViTri: false }
```

Hai dấu hiệu KHÁC nhau tố cáo cùng MỘT lỗi: sector `2` (đích ĐÚNG)
`dungViTri:false` VÌ nó chưa hề nhận được ghi mới nào (vẫn `0,0,0` mặc
định — đáng ngờ VÌ lẽ ra phải có `8,8,8`); sector `1` (nơi dữ liệu
THỰC SỰ hạ cánh) cũng `dungViTri:false` VÌ tag bên trong nó nói "tôi
LÀ sector 2" trong khi nó nằm Ở sector `1`. Một checksum bình thường
(tính TRÊN payload, không biết gì về VỊ TRÍ) sẽ khớp HOÀN hảo ở cả hai
nơi — hoàn toàn KHÔNG phát hiện được gì.
::::

::::example{#vi-sao-tigerbeetle-quan-tam}
Đây LÀ đúng lý do các hệ thống lưu trữ nghiêm túc (bao gồm TigerBeetle)
nhúng "checksum CỦA checksum" và ID vị trí NGAY trong mỗi bản ghi, thay
VÌ chỉ tin "đọc được LÀ đúng". Ghi sai địa chỉ LÀ một trong những lỗi
đĩa khó phát hiện nhất trong thực tế — chính vì dữ liệu VẪN hợp lệ,
chỉ SAI chỗ.
::::

::::predict{#doan-ghi-hai-lan-cung-dich commitOnce}
Sau đoạn code trên, gọi THÊM `ghiCoNhan(disk, 1, [3,3,3])` rồi
`disk.fsync()` (không có `danhDauGhiSaiDich` nào mới). `docCoKiemTraViTri
(disk, 1)` sau đó CÓ trả về `dungViTri: true` không?
:::opt{correct}
CÓ — `danhDauGhiSaiDich` LÀ một "bẫy một lần", tự xoá SAU đúng một lần
`fsync()` (dòng `this.ghiSaiDichKeTiep = null;` chạy TRƯỚC vòng lặp
ghi) — lần `fsync()` NÀY hoàn toàn bình thường, `ghiCoNhan(disk,1,...)`
tự nhúng tag `1` ĐÚNG vị trí của nó
:::
:::opt
Không — MỘT khi `sectorThucTe=1` đã bị ghi nhầm một lần, sector đó
"hỏng" cho MỌI lần ghi sau
::why
Trực giác NÀY nhầm "một sector đã TỪNG bị ảnh hưởng bởi lỗi" VỚI "một
sector VĨNH VIỄN mang lỗi" — nhưng `danhDauGhiSaiDich` không hề đổi gì
Ở BẢN THÂN sector 1, nó chỉ chuyển hướng đúng MỘT lần ghi kế tiếp.

Chỗ lệch: `ghiSaiDichKeTiep` bị gán về `null` NGAY khi `fsync()` bắt
đầu xử lý — bất kỳ lần `write`/`fsync` NÀO sau đó đều đi ĐÚNG đường,
không còn bị chuyển hướng nữa.
::
:::
::::

::::code{#viet_doc_co_kiem_tra_vi_tri}
Hoàn thiện `docCoKiemTraViTri` — `dungViTri` LÀ `true` khi VÀ chỉ khi
byte đầu tiên của bản ghi (`bytes[0]`) bằng đúng `sector` đang đọc.

```typescript title=starter
class SimDisk {
  readonly kichThuocSector: number;
  private readonly platter = new Map<number, Uint8Array>();
  private readonly cache = new Map<number, Uint8Array>();
  private readonly sectorLoiAn = new Set<number>();
  private ghiSaiDichKeTiep: { dinh: number; thuc: number } | null = null;
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  read(sector: number): Uint8Array {
    if (this.sectorLoiAn.has(sector)) throw new Error(`latent sector error: sector ${sector}`);
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }
  write(sector: number, data: Uint8Array): void { this.cache.set(sector, data.slice()); }
  fsync(): void {
    const saiDich = this.ghiSaiDichKeTiep;
    this.ghiSaiDichKeTiep = null;
    for (const [sector, data] of this.cache) {
      if (saiDich !== null && sector === saiDich.dinh) {
        this.platter.set(saiDich.thuc, data);
        this.sectorLoiAn.delete(saiDich.thuc);
        continue;
      }
      this.platter.set(sector, data);
      this.sectorLoiAn.delete(sector);
    }
    this.cache.clear();
  }
  danhDauGhiSaiDich(sectorDinhGhi: number, sectorThucTe: number): void {
    this.ghiSaiDichKeTiep = { dinh: sectorDinhGhi, thuc: sectorThucTe };
  }
}

function ghiCoNhan(disk: SimDisk, sector: number, payload: number[]): void {
  const buf = new Uint8Array(disk.kichThuocSector);
  buf[0] = sector;
  buf.set(payload, 1);
  disk.write(sector, buf);
}
function docCoKiemTraViTri(disk: SimDisk, sector: number): { duLieu: number[]; dungViTri: boolean } {
  const bytes = disk.read(sector);
  ___
}

const disk = new SimDisk(4);
ghiCoNhan(disk, 1, [7, 7, 7]);
disk.fsync();
disk.danhDauGhiSaiDich(2, 1);
ghiCoNhan(disk, 2, [8, 8, 8]);
disk.fsync();
console.log(docCoKiemTraViTri(disk, 1));
```

```typescript title=solution
class SimDisk {
  readonly kichThuocSector: number;
  private readonly platter = new Map<number, Uint8Array>();
  private readonly cache = new Map<number, Uint8Array>();
  private readonly sectorLoiAn = new Set<number>();
  private ghiSaiDichKeTiep: { dinh: number; thuc: number } | null = null;
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  read(sector: number): Uint8Array {
    if (this.sectorLoiAn.has(sector)) throw new Error(`latent sector error: sector ${sector}`);
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }
  write(sector: number, data: Uint8Array): void { this.cache.set(sector, data.slice()); }
  fsync(): void {
    const saiDich = this.ghiSaiDichKeTiep;
    this.ghiSaiDichKeTiep = null;
    for (const [sector, data] of this.cache) {
      if (saiDich !== null && sector === saiDich.dinh) {
        this.platter.set(saiDich.thuc, data);
        this.sectorLoiAn.delete(saiDich.thuc);
        continue;
      }
      this.platter.set(sector, data);
      this.sectorLoiAn.delete(sector);
    }
    this.cache.clear();
  }
  danhDauGhiSaiDich(sectorDinhGhi: number, sectorThucTe: number): void {
    this.ghiSaiDichKeTiep = { dinh: sectorDinhGhi, thuc: sectorThucTe };
  }
}

function ghiCoNhan(disk: SimDisk, sector: number, payload: number[]): void {
  const buf = new Uint8Array(disk.kichThuocSector);
  buf[0] = sector;
  buf.set(payload, 1);
  disk.write(sector, buf);
}
function docCoKiemTraViTri(disk: SimDisk, sector: number): { duLieu: number[]; dungViTri: boolean } {
  const bytes = disk.read(sector);
  return { duLieu: [...bytes.slice(1)], dungViTri: bytes[0] === sector };
}

const disk = new SimDisk(4);
ghiCoNhan(disk, 1, [7, 7, 7]);
disk.fsync();
disk.danhDauGhiSaiDich(2, 1);
ghiCoNhan(disk, 2, [8, 8, 8]);
disk.fsync();
console.log(docCoKiemTraViTri(disk, 1));
```

```typescript title=test
const diskT = new SimDisk(4);
ghiCoNhan(diskT, 1, [7, 7, 7]);
diskT.fsync();
const truoc = docCoKiemTraViTri(diskT, 1);
if (!truoc.dungViTri) throw new Error("truoc khi bi ghi sai dich, sector 1 phai dungViTri=true");
if (JSON.stringify(truoc.duLieu) !== "[7,7,7]") throw new Error("truoc khi bi ghi sai dich, du lieu sector 1 phai la [7,7,7]");

diskT.danhDauGhiSaiDich(2, 1);
ghiCoNhan(diskT, 2, [8, 8, 8]);
diskT.fsync();

const sauSector2 = docCoKiemTraViTri(diskT, 2);
if (sauSector2.dungViTri) throw new Error("sector 2 (dinh ghi, bi redirect) khong he duoc ghi that -- dungViTri phai la false");

const sauSector1 = docCoKiemTraViTri(diskT, 1);
if (sauSector1.dungViTri) throw new Error("sector 1 (thuc te nhan du lieu nham) phai dungViTri=false -- tag ben trong noi la sector 2");
if (JSON.stringify(sauSector1.duLieu) !== "[8,8,8]") throw new Error("sector 1 phai chua du lieu THAT SU bi ghi nham, [8,8,8]");

ghiCoNhan(diskT, 1, [3, 3, 3]);
diskT.fsync();
const sauGhiLaiDungCho = docCoKiemTraViTri(diskT, 1);
if (!sauGhiLaiDungCho.dungViTri) throw new Error("danhDauGhiSaiDich la bay MOT LAN -- ghi lai binh thuong sau do phai dungViTri=true");
if (JSON.stringify(sauGhiLaiDungCho.duLieu) !== "[3,3,3]") throw new Error("sau ghi lai dung cho, du lieu phai la [3,3,3]");
```

:::hints
- kind: attention
  body: "dungViTri la bytes[0] === sector; duLieu la bytes.slice(1) -- mot dong return."
- kind: strategy
  body: "return { duLieu: [...bytes.slice(1)], dungViTri: bytes[0] === sector };"
- kind: one-line
  body: "return { duLieu: [...bytes.slice(1)], dungViTri: bytes[0] === sector };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "dungViTri"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba lỗi đĩa xong: lost fsync/torn write (bài 2), latent sector error
(bài 3), misdirected write (bài này). Còn một câu hỏi: CHÍNH XÁC khi
nào một crash xảy ra?
::::

::::reflect{#nghi-lai}
`docCoKiemTraViTri` không thêm phép toán MỚI — nó áp dụng lại đúng ý
tưởng "bản ghi tự nhận diện" từ q18 bài 1 (chọn mục tiêu bằng seed) và
mở rộng NÓ thành "bản ghi tự XÁC MINH vị trí của chính nó". Bài học lớn
nhất: một checksum THÔNG THƯỜNG bảo vệ NỘI DUNG nhưng không bảo vệ VỊ
TRÍ — muốn bắt được misdirected write, vị trí phải LÀ một phần của nội
dung.
::::

::::checkpoint{mastery=0.85}
::::
