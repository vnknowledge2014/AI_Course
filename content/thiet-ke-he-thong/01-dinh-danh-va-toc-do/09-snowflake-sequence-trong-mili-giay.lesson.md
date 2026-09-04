---
id: thiet-ke-he-thong.dinh-danh-va-toc-do.snowflake-sequence-trong-mili-giay
title: "Snowflake ID: sequence trong cùng mili-giây"
summary: "sinhSnowflakeId tăng sequence 0,1,2... cho các ID sinh ra CÙNG một mili-giây (so msHienTai với bo.msCuoiCung), reset sequence về 0 khi sang mili-giây mới. Tràn sequence (>4095 trong CÙNG ms) tự tiến đồng hồ ảo (vòng while gọi tienThoiGian) tới mili-giây kế tiếp rồi mới cấp tiếp -- 4097 lệnh sinh ID liên tiếp KHÔNG hề gọi tienThoiGian từ bên ngoài: 4096 ID đầu nằm trọn trong ms=500 (sequence 0..4095), ID thứ 4097 tự động rơi sang ms=501, sequence về 0."
locale: vi
track: thiet-ke-he-thong
module: dinh-danh-va-toc-do
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.snowflake-sequence-trong-mili-giay]
requires: [sd.snowflake-dong-goi-bit]
concepts: [sd.snowflake-sequence-trong-mili-giay]
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
Bài trước ghép bit ĐÚNG, nhưng `ghepSnowflakeId` chỉ nhận sequence
NHƯ một tham số CÓ SẴN — không hề tự QUẢN nó. Ai chịu trách nhiệm
tăng sequence, VÀ khi nào phải reset nó VỀ 0?
::::

::::explain{#quan-ly-sequence}
Một "bộ SINH" (`BoSinhSnowflake`) nhớ `msCuoiCung` (mili-giây của
lần sinh gần NHẤT) VÀ `sequenceHienTai`. Sinh ID MỚI: nếu mili-giây
HIỆN tại TRÙNG với `msCuoiCung`, tăng sequence LÊN `1`; nếu KHÁC
(sang mili-giây mới), reset sequence VỀ `0`:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

const SO_BIT_SEQUENCE = 12n;
const SO_BIT_MACHINE = 5n;
const SO_BIT_DATACENTER = 5n;
const DICH_MACHINE = SO_BIT_SEQUENCE;
const DICH_DATACENTER = SO_BIT_MACHINE + SO_BIT_SEQUENCE;
const DICH_TIMESTAMP = SO_BIT_DATACENTER + SO_BIT_MACHINE + SO_BIT_SEQUENCE;
function ghepSnowflakeId(msTuEpoch: bigint, datacenterId: bigint, machineId: bigint, sequence: bigint): bigint {
  return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;
}

const SEQUENCE_TOI_DA = 4095;

interface BoSinhSnowflake { datacenterId: bigint; machineId: bigint; msCuoiCung: number; sequenceHienTai: number; }
function taoBoSinhSnowflake(datacenterId: bigint, machineId: bigint): BoSinhSnowflake {
  return { datacenterId, machineId, msCuoiCung: -1, sequenceHienTai: 0 };
}
function sinhSnowflakeId(bo: BoSinhSnowflake, dh: DongHoMoPhong): bigint {
  let msHienTai = dh.thoiGianHienTai;
  if (msHienTai === bo.msCuoiCung) {
    bo.sequenceHienTai++;
    if (bo.sequenceHienTai > SEQUENCE_TOI_DA) {
      while (dh.thoiGianHienTai === bo.msCuoiCung) tienThoiGian(dh, 1);
      msHienTai = dh.thoiGianHienTai;
      bo.sequenceHienTai = 0;
    }
  } else {
    bo.sequenceHienTai = 0;
  }
  bo.msCuoiCung = msHienTai;
  return ghepSnowflakeId(BigInt(msHienTai), bo.datacenterId, bo.machineId, BigInt(bo.sequenceHienTai));
}

const dh = taoDongHoMoPhong();
const bo = taoBoSinhSnowflake(1n, 1n);

dh.thoiGianHienTai = 100;
const id1 = sinhSnowflakeId(bo, dh);
const id2 = sinhSnowflakeId(bo, dh);
const id3 = sinhSnowflakeId(bo, dh);
console.log("3 ID sinh CUNG mot ms (t=100):");
console.log("  id1:", String(id1));
console.log("  id2:", String(id2));
console.log("  id3:", String(id3));
console.log("  id2 - id1 =", String(id2 - id1), ", id3 - id2 =", String(id3 - id2), "(sequence tang dan 1 don vi)");
console.log("  bo.sequenceHienTai cuoi cung:", bo.sequenceHienTai, "-- bo.msCuoiCung:", bo.msCuoiCung);

dh.thoiGianHienTai = 101;
const id4 = sinhSnowflakeId(bo, dh);
console.log("sang ms moi (t=101): bo.sequenceHienTai reset ve:", bo.sequenceHienTai);
console.log("  id4:", String(id4), "-- id4 > id3:", id4 > id3);
```

```text title=readonly
3 ID sinh CUNG mot ms (t=100):
  id1: 419565568
  id2: 419565569
  id3: 419565570
  id2 - id1 = 1 , id3 - id2 = 1 (sequence tang dan 1 don vi)
  bo.sequenceHienTai cuoi cung: 2 -- bo.msCuoiCung: 100
sang ms moi (t=101): bo.sequenceHienTai reset ve: 0
  id4: 423759872 -- id4 > id3: true
```

Ba lần gọi `sinhSnowflakeId` LIÊN tiếp Ở CÙNG `t=100`: `msHienTai ===
bo.msCuoiCung` đúng TỪ lần gọi thứ HAI trở đi, `sequenceHienTai` tăng
DẦN `0 → 1 → 2`. Sang `t=101` (khác `100`), điều kiện SAI, sequence
reset VỀ `0` — VÀ dù reset, `id4` vẫn LỚN hơn `id3`, VÌ timestamp
(bit CAO nhất) đã tăng, áp ĐẢO mọi ảnh hưởng từ việc sequence bị hạ
xuống.
::::

::::example{#tran-sequence}
`sequence` CHỈ có `12` bit — TỐI đa `4096` giá trị (`0` tới `4095`)
MỖI mili-giây MỖI máy. Nếu CẦN sinh ID thứ `4097` TRONG cùng một
mili-giây, KHÔNG còn chỗ — bộ sinh phải TỰ tiến đồng hồ ảo sang
mili-giây KẾ tiếp rồi mới cấp tiếp, hoàn toàn KHÔNG cần chờ thời gian
THẬT:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

const SO_BIT_SEQUENCE = 12n;
const SO_BIT_MACHINE = 5n;
const SO_BIT_DATACENTER = 5n;
const DICH_MACHINE = SO_BIT_SEQUENCE;
const DICH_DATACENTER = SO_BIT_MACHINE + SO_BIT_SEQUENCE;
const DICH_TIMESTAMP = SO_BIT_DATACENTER + SO_BIT_MACHINE + SO_BIT_SEQUENCE;
function ghepSnowflakeId(msTuEpoch: bigint, datacenterId: bigint, machineId: bigint, sequence: bigint): bigint {
  return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;
}

const SEQUENCE_TOI_DA = 4095;

interface BoSinhSnowflake { datacenterId: bigint; machineId: bigint; msCuoiCung: number; sequenceHienTai: number; }
function taoBoSinhSnowflake(datacenterId: bigint, machineId: bigint): BoSinhSnowflake {
  return { datacenterId, machineId, msCuoiCung: -1, sequenceHienTai: 0 };
}
function sinhSnowflakeId(bo: BoSinhSnowflake, dh: DongHoMoPhong): bigint {
  let msHienTai = dh.thoiGianHienTai;
  if (msHienTai === bo.msCuoiCung) {
    bo.sequenceHienTai++;
    if (bo.sequenceHienTai > SEQUENCE_TOI_DA) {
      while (dh.thoiGianHienTai === bo.msCuoiCung) tienThoiGian(dh, 1);
      msHienTai = dh.thoiGianHienTai;
      bo.sequenceHienTai = 0;
    }
  } else {
    bo.sequenceHienTai = 0;
  }
  bo.msCuoiCung = msHienTai;
  return ghepSnowflakeId(BigInt(msHienTai), bo.datacenterId, bo.machineId, BigInt(bo.sequenceHienTai));
}

// sinh 4097 ID LIEN TIEP, KHONG bao gio tu goi tienThoiGian tu ben ngoai -- dong ho CHI tien
// khi chinh sinhSnowflakeId can (tran sequence)
const dh = taoDongHoMoPhong();
dh.thoiGianHienTai = 500;
const bo = taoBoSinhSnowflake(0n, 0n);

const cacId: bigint[] = [];
for (let i = 0; i < 4097; i++) cacId.push(sinhSnowflakeId(bo, dh));

console.log("tong so ID da sinh:", cacId.length);
console.log("dong ho SAU khi sinh xong:", dh.thoiGianHienTai, "(bat dau tu 500, khong ai tien no tu ben ngoai)");
console.log("id dau (index 0) va id thu 4096 (index 4095) chenh lech sequence dung 4095:", cacId[4095]! - cacId[0]! === 4095n);
console.log("ID thu 4097 (index 4096, VUOT qua 4096 ID/ms) sequence phai VE 0, o ms KE tiep:");
console.log("  id thu 4096 (index 4095):", String(cacId[4095]));
console.log("  id thu 4097 (index 4096):", String(cacId[4096]));
console.log("  chenh lech (id 4097 - id 4096):", String(cacId[4096]! - cacId[4095]!));

const soLuongTaiMs500 = cacId.filter((id) => {
  const seq = id & 0xfffn;
  return id === ghepSnowflakeId(500n, 0n, 0n, seq);
}).length;
console.log("so ID nam trong dung ms=500:", soLuongTaiMs500, "(dung bang gioi han 4096 ID/ms/may)");
```

```text title=readonly
tong so ID da sinh: 4097
dong ho SAU khi sinh xong: 501 (bat dau tu 500, khong ai tien no tu ben ngoai)
id dau (index 0) va id thu 4096 (index 4095) chenh lech sequence dung 4095: true
ID thu 4097 (index 4096, VUOT qua 4096 ID/ms) sequence phai VE 0, o ms KE tiep:
  id thu 4096 (index 4095): 2097156095
  id thu 4097 (index 4096): 2101346304
  chenh lech (id 4097 - id 4096): 4190209
so ID nam trong dung ms=500: 4096 (dung bang gioi han 4096 ID/ms/may)
```

`4096` ID đầu (index `0` tới `4095`) nằm TRỌN trong `ms=500`, dùng
HẾT toàn bộ khoảng sequence `0..4095`. ID thứ `4097` gặp
`sequenceHienTai > SEQUENCE_TOI_DA`, kích HOẠT vòng `while` — TỰ gọi
`tienThoiGian(dh, 1)` (không CẦN ai gọi TỪ bên ngoài) cho tới KHI
đồng hồ khác `msCuoiCung`, đưa `dh.thoiGianHienTai` LÊN `501`. Toàn
bộ diễn ra "tức THÌ" trong mô phỏng — không CÓ độ trễ thời gian THẬT
nào.
::::

::::predict{#doan-sequence-vua-du commitOnce}
Một bộ sinh vừa CẤP ID với `sequenceHienTai = 4095` (ĐÚNG giới hạn
`SEQUENCE_TOI_DA`), VẪN Ở CÙNG mili-giây. Gọi `sinhSnowflakeId` thêm
đúng MỘT lần nữa (không đổi `dh.thoiGianHienTai` TỪ bên ngoài). Lần
gọi NÀY có kích hoạt việc TIẾN đồng hồ sang mili-giây KẾ tiếp không?

:::opt{correct}
CÓ — `sequenceHienTai` tăng LÊN `4096` (VƯỢT `SEQUENCE_TOI_DA = 4095`),
điều kiện tràn ĐÚNG, vòng `while` kích hoạt VÀ tự tiến đồng hồ
:::
:::opt
KHÔNG — `sequenceHienTai` ĐÚNG bằng `SEQUENCE_TOI_DA` (chưa VƯỢT quá
`4095`), nên LẦN gọi tiếp theo vẫn CÒN nằm gọn trong giới hạn
::why
Nhầm "GIÁ TRỊ hiện tại BẰNG giới hạn" VỚI "GIÁ TRỊ SAU khi tăng vẫn
nằm trong giới hạn" — nhưng `sinhSnowflakeId` TĂNG `sequenceHienTai`
LÊN TRƯỚC rồi mới kiểm tra tràn.

Chỗ lệch: dòng ĐẦU trong nhánh "cùng ms" LÀ `bo.sequenceHienTai++;`
— thực hiện TRƯỚC khi so sánh. Từ `4095`, sau `++` NÓ thành `4096`,
VÀ `4096 > SEQUENCE_TOI_DA (4095)` LÀ đúng — tràn XẢY ra NGAY Ở lần
gọi kế tiếp NÀY, không cần đợi thêm request NÀO nữa.
::
:::
::::

::::code{#viet_sinh_snowflake_id}
Hoàn thiện phần XỬ lý tràn sequence trong `sinhSnowflakeId` — khi
`sequenceHienTai` VƯỢT `SEQUENCE_TOI_DA`, tiến đồng hồ ẢO (dùng
`tienThoiGian`) CHO tới khi sang mili-giây MỚI.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

const SO_BIT_SEQUENCE = 12n;
const SO_BIT_MACHINE = 5n;
const SO_BIT_DATACENTER = 5n;
const DICH_MACHINE = SO_BIT_SEQUENCE;
const DICH_DATACENTER = SO_BIT_MACHINE + SO_BIT_SEQUENCE;
const DICH_TIMESTAMP = SO_BIT_DATACENTER + SO_BIT_MACHINE + SO_BIT_SEQUENCE;
function ghepSnowflakeId(msTuEpoch: bigint, datacenterId: bigint, machineId: bigint, sequence: bigint): bigint {
  return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;
}

const SEQUENCE_TOI_DA = 4095;

interface BoSinhSnowflake { datacenterId: bigint; machineId: bigint; msCuoiCung: number; sequenceHienTai: number; }
function taoBoSinhSnowflake(datacenterId: bigint, machineId: bigint): BoSinhSnowflake {
  return { datacenterId, machineId, msCuoiCung: -1, sequenceHienTai: 0 };
}
function sinhSnowflakeId(bo: BoSinhSnowflake, dh: DongHoMoPhong): bigint {
  let msHienTai = dh.thoiGianHienTai;
  if (msHienTai === bo.msCuoiCung) {
    bo.sequenceHienTai++;
    if (bo.sequenceHienTai > SEQUENCE_TOI_DA) {
      ___
      msHienTai = dh.thoiGianHienTai;
      bo.sequenceHienTai = 0;
    }
  } else {
    bo.sequenceHienTai = 0;
  }
  bo.msCuoiCung = msHienTai;
  return ghepSnowflakeId(BigInt(msHienTai), bo.datacenterId, bo.machineId, BigInt(bo.sequenceHienTai));
}

const dh = taoDongHoMoPhong();
dh.thoiGianHienTai = 10;
const bo = taoBoSinhSnowflake(0n, 0n);
console.log(String(sinhSnowflakeId(bo, dh)), String(sinhSnowflakeId(bo, dh)));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

const SO_BIT_SEQUENCE = 12n;
const SO_BIT_MACHINE = 5n;
const SO_BIT_DATACENTER = 5n;
const DICH_MACHINE = SO_BIT_SEQUENCE;
const DICH_DATACENTER = SO_BIT_MACHINE + SO_BIT_SEQUENCE;
const DICH_TIMESTAMP = SO_BIT_DATACENTER + SO_BIT_MACHINE + SO_BIT_SEQUENCE;
function ghepSnowflakeId(msTuEpoch: bigint, datacenterId: bigint, machineId: bigint, sequence: bigint): bigint {
  return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;
}

const SEQUENCE_TOI_DA = 4095;

interface BoSinhSnowflake { datacenterId: bigint; machineId: bigint; msCuoiCung: number; sequenceHienTai: number; }
function taoBoSinhSnowflake(datacenterId: bigint, machineId: bigint): BoSinhSnowflake {
  return { datacenterId, machineId, msCuoiCung: -1, sequenceHienTai: 0 };
}
function sinhSnowflakeId(bo: BoSinhSnowflake, dh: DongHoMoPhong): bigint {
  let msHienTai = dh.thoiGianHienTai;
  if (msHienTai === bo.msCuoiCung) {
    bo.sequenceHienTai++;
    if (bo.sequenceHienTai > SEQUENCE_TOI_DA) {
      while (dh.thoiGianHienTai === bo.msCuoiCung) tienThoiGian(dh, 1);
      msHienTai = dh.thoiGianHienTai;
      bo.sequenceHienTai = 0;
    }
  } else {
    bo.sequenceHienTai = 0;
  }
  bo.msCuoiCung = msHienTai;
  return ghepSnowflakeId(BigInt(msHienTai), bo.datacenterId, bo.machineId, BigInt(bo.sequenceHienTai));
}

const dh = taoDongHoMoPhong();
dh.thoiGianHienTai = 10;
const bo = taoBoSinhSnowflake(0n, 0n);
console.log(String(sinhSnowflakeId(bo, dh)), String(sinhSnowflakeId(bo, dh)));
```

```typescript title=test
function layMsCuoiCung(bo: BoSinhSnowflake): number { return bo.msCuoiCung; }
function laySequenceHienTai(bo: BoSinhSnowflake): number { return bo.sequenceHienTai; }

const dhT1 = taoDongHoMoPhong();
dhT1.thoiGianHienTai = 200;
const boT1 = taoBoSinhSnowflake(0n, 0n);
const idA = sinhSnowflakeId(boT1, dhT1);
const idB = sinhSnowflakeId(boT1, dhT1);
const idC = sinhSnowflakeId(boT1, dhT1);
if (idB - idA !== 1n) throw new Error("ID thu 2 cung ms phai hon ID thu 1 dung 1 (sequence 0 -> 1)");
if (idC - idB !== 1n) throw new Error("ID thu 3 cung ms phai hon ID thu 2 dung 1 (sequence 1 -> 2)");
if (laySequenceHienTai(boT1) !== 2) throw new Error("sau 3 lan sinh cung ms, sequenceHienTai phai la 2");
if (layMsCuoiCung(boT1) !== 200) throw new Error("msCuoiCung phai giu nguyen 200 (chua tran, chua sang ms moi)");

dhT1.thoiGianHienTai = 201;
const idD = sinhSnowflakeId(boT1, dhT1);
if (laySequenceHienTai(boT1) !== 0) throw new Error("sang ms MOI qua duong binh thuong (khong phai do tran), sequenceHienTai (dang la 2) PHAI duoc RESET ve 0 -- nhanh else khong duoc BO SOT dong nay");
if (layMsCuoiCung(boT1) !== 201) throw new Error("msCuoiCung phai cap nhat thanh 201");
if (idD <= idC) throw new Error("idD (ms=201, seq=0) phai lon hon idC (ms=200, seq=2)");

const dhT2 = taoDongHoMoPhong();
dhT2.thoiGianHienTai = 500;
const boT2 = taoBoSinhSnowflake(0n, 0n);
const cacId: bigint[] = [];
for (let i = 0; i < 4097; i++) cacId.push(sinhSnowflakeId(boT2, dhT2));
if (dhT2.thoiGianHienTai !== 501) throw new Error("dong ho phai TU tien len 501 (tran sequence buoc phai cho sang ms tiep theo)");
if (layMsCuoiCung(boT2) !== 501) throw new Error("msCuoiCung phai cap nhat thanh 501 sau khi tran");
if (laySequenceHienTai(boT2) !== 0) throw new Error("sequenceHienTai phai RESET ve 0 ngay sau khi sang ms moi vi tran");
if (cacId[4095]! - cacId[0]! !== 4095n) throw new Error("4096 ID dau (index 0..4095) phai dung het het sequence 0..4095 trong ms=500");
if (cacId[4096]! <= cacId[4095]!) throw new Error("ID thu 4097 (sau khi tran, sang ms moi) phai LON hon ID thu 4096");
```

:::hints
- kind: attention
  body: "Lap while: chung nao dong ho van con o dung ms cua bo.msCuoiCung, tien no len 1ms -- mot dong."
- kind: strategy
  body: "while (dh.thoiGianHienTai === bo.msCuoiCung) tienThoiGian(dh, 1);"
- kind: one-line
  body: "while (dh.thoiGianHienTai === bo.msCuoiCung) tienThoiGian(dh, 1);"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "41943041"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Rate limiting (4 thuật toán) VÀ Snowflake ID (đóng gói bit, quản
sequence) — cả hai mảnh đã xong RIÊNG lẻ. BOSS ráp CHÚNG vào MỘT
cổng API duy nhất.
::::

::::reflect{#nghi-lai}
`sinhSnowflakeId` chỉ CÓ đúng hai nhánh (cùng ms / khác ms) VÀ một
vòng `while` xử lý tràn — nhưng thứ tự "tăng TRƯỚC, kiểm tra SAU"
chính LÀ điều làm `4096` ID/ms/máy trở thành một giới HẠN cứng, không
BAO giờ bị vượt: đến ĐÚNG ID thứ `4097`, hệ thống TỰ biết mình PHẢI
đợi sang mili-giây kế tiếp.
::::

::::checkpoint{mastery=0.8}
::::
