---
id: thiet-ke-he-thong.ha-tang-du-lieu-quy-mo-lon.metrics-gom-theo-khung-thoi-gian
title: "Gom metric theo khung: đừng giữ từng điểm thô"
summary: "ghiDiemTho(bo, dh, giaTri) khong luu tung diem tho rieng le -- no CONG don gia tri vao dung khung thoi gian (chiSoKhung = Math.floor(thoiGian / KHUNG_MS)), tang soDiem. 4 diem tho ghi trong khung 0 (t=0..8000) chi tao DUNG 1 khung trong bo nho, trungBinhKhung tra ve 25 (trung binh cong). Sang khung 1 (t=10000) them 2 diem nua -- tong 6 diem tho nhung chi 2 khung duoc luu, khung 0 khong doi."
locale: vi
track: thiet-ke-he-thong
module: ha-tang-du-lieu-quy-mo-lon
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.metrics-gom-theo-khung-thoi-gian]
requires: [sd.message-queue-xac-nhan-va-gui-lai]
concepts: [sd.metrics-gom-theo-khung-thoi-gian]
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
Hàng đợi tin nhắn xong. Mảnh HẠ tầng thứ hai: một hệ thống theo dõi
(monitoring) nhận về HÀNG nghìn điểm dữ liệu thô mỗi giây — CPU, độ
trễ, số yêu cầu. Giữ NGUYÊN từng điểm một thì bộ nhớ nổ tung. Câu hỏi
đầu tiên: NÉN chúng lại như thế nào?
::::

::::explain{#gom-theo-khung}
`ghiDiemTho` không lưu từng điểm thô riêng LẺ — nó xác định điểm đó
thuộc khung thời gian NÀO (`chiSoKhung`, chia đều theo `KHUNG_MS`),
RỒI cộng dồn giá trị VÀO đúng khung đó VÀ tăng `soDiem`. Nhiều điểm thô
rơi CÙNG một khung chỉ tốn ĐÚNG một mục trong bộ nhớ:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

const KHUNG_MS = 10000;
interface KhungGom { tong: number; soDiem: number; }
interface BoGomMetric { khung: Map<number, KhungGom>; }
function taoBoGomMetric(): BoGomMetric { return { khung: new Map() }; }
function chiSoKhung(thoiGian: number): number { return Math.floor(thoiGian / KHUNG_MS); }
function ghiDiemTho(bo: BoGomMetric, dh: DongHoMoPhong, giaTri: number): void {
  const idx = chiSoKhung(dh.thoiGianHienTai);
  const hienTai = bo.khung.get(idx) ?? { tong: 0, soDiem: 0 };
  hienTai.tong += giaTri;
  hienTai.soDiem += 1;
  bo.khung.set(idx, hienTai);
}
function trungBinhKhung(bo: BoGomMetric, idx: number): number | undefined {
  const k = bo.khung.get(idx);
  if (k === undefined) return undefined;
  return k.tong / k.soDiem;
}
function tongSoDiemDaGhi(bo: BoGomMetric): number {
  let t = 0;
  for (const k of bo.khung.values()) t += k.soDiem;
  return t;
}

const dh = taoDongHoMoPhong();
const bo = taoBoGomMetric();
ghiDiemTho(bo, dh, 10);
tienThoiGian(dh, 2000);
ghiDiemTho(bo, dh, 20);
tienThoiGian(dh, 3000);
ghiDiemTho(bo, dh, 30);
tienThoiGian(dh, 3000);
ghiDiemTho(bo, dh, 40);
console.log("da ghi 4 diem tho, thoi gian hien tai:", dh.thoiGianHienTai);
console.log("trung binh khung 0:", trungBinhKhung(bo, 0));
console.log("so khung da luu (bo nho):", bo.khung.size);
console.log("tong so diem tho da nen vao cac khung:", tongSoDiemDaGhi(bo));
```

```text title=readonly
da ghi 4 diem tho, thoi gian hien tai: 8000
trung binh khung 0: 25
so khung da luu (bo nho): 1
tong so diem tho da nen vao cac khung: 4
```

Bốn điểm thô (`10, 20, 30, 40`) đều rơi trong khoảng `t=0` tới `t=8000`
— cùng khung `0` (vì `KHUNG_MS = 10000`) — NÊN `ghiDiemTho` chỉ tạo
ĐÚNG một mục trong `Map`. `trungBinhKhung` trả VỀ `25`, đúng bằng trung
bình cộng CỦA cả bốn điểm — nhưng bộ nhớ chỉ giữ MỘT con số tổng VÀ một
bộ đếm, không giữ RIÊNG bốn điểm gốc.
::::

::::example{#nen-vao-nhieu-khung}
Khi đồng hồ bước SANG khung thời gian tiếp theo, `ghiDiemTho` tạo một
mục MỚI — khung CŨ không hề bị đụng tới, VÀ số lượng khung trong bộ
nhớ tăng CHẬM hơn HẲN số điểm thô đã ghi:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

const KHUNG_MS = 10000;
interface KhungGom { tong: number; soDiem: number; }
interface BoGomMetric { khung: Map<number, KhungGom>; }
function taoBoGomMetric(): BoGomMetric { return { khung: new Map() }; }
function chiSoKhung(thoiGian: number): number { return Math.floor(thoiGian / KHUNG_MS); }
function ghiDiemTho(bo: BoGomMetric, dh: DongHoMoPhong, giaTri: number): void {
  const idx = chiSoKhung(dh.thoiGianHienTai);
  const hienTai = bo.khung.get(idx) ?? { tong: 0, soDiem: 0 };
  hienTai.tong += giaTri;
  hienTai.soDiem += 1;
  bo.khung.set(idx, hienTai);
}
function trungBinhKhung(bo: BoGomMetric, idx: number): number | undefined {
  const k = bo.khung.get(idx);
  if (k === undefined) return undefined;
  return k.tong / k.soDiem;
}
function tongSoDiemDaGhi(bo: BoGomMetric): number {
  let t = 0;
  for (const k of bo.khung.values()) t += k.soDiem;
  return t;
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: 4 diem tho (10,20,30,40)
// da ghi trong khung 0, dong ho dang o t=8000
const dh = taoDongHoMoPhong();
const bo = taoBoGomMetric();
ghiDiemTho(bo, dh, 10);
tienThoiGian(dh, 2000);
ghiDiemTho(bo, dh, 20);
tienThoiGian(dh, 3000);
ghiDiemTho(bo, dh, 30);
tienThoiGian(dh, 3000);
ghiDiemTho(bo, dh, 40);

tienThoiGian(dh, 2000); // t = 10000, sang khung 1
console.log("thoi gian hien tai:", dh.thoiGianHienTai, "-> chi so khung:", chiSoKhung(dh.thoiGianHienTai));
ghiDiemTho(bo, dh, 100);
tienThoiGian(dh, 1000);
ghiDiemTho(bo, dh, 200);

console.log("trung binh khung 0 (khong doi):", trungBinhKhung(bo, 0));
console.log("trung binh khung 1:", trungBinhKhung(bo, 1));
console.log("trung binh khung 2 (chua co diem nao):", trungBinhKhung(bo, 2));
console.log("tong so diem tho da ghi:", tongSoDiemDaGhi(bo));
console.log("so khung da luu (bo nho thuc te):", bo.khung.size);
```

```text title=readonly
thoi gian hien tai: 10000 -> chi so khung: 1
trung binh khung 0 (khong doi): 25
trung binh khung 1: 150
trung binh khung 2 (chua co diem nao): undefined
tong so diem tho da ghi: 6
so khung da luu (bo nho thuc te): 2
```

`6` điểm thô đã được ghi TỔNG cộng, nhưng bộ nhớ chỉ giữ `2` khung —
đây chính LÀ khoản tiết kiệm mà việc gom mang lại. Khung `0` giữ
NGUYÊN giá trị `25` dù đồng hồ đã trôi RẤT xa khỏi nó. Khung `2` chưa
hề có điểm nào NÊN `trungBinhKhung` trả về `undefined`, không phải `0`
— tránh nhầm "chưa có dữ liệu" với "dữ liệu bằng không".
::::

::::predict{#doan-ranh-gioi-khung commitOnce}
Đồng hồ đang Ở `t=9999` (còn TRONG khung `0`, vì `KHUNG_MS = 10000`).
`ghiDiemTho` ghi giá trị `50` NGAY tại đó. Đồng hồ tiến thêm ĐÚNG
`1ms` (`t=10000`), RỒI `ghiDiemTho` ghi giá trị `60`. Bộ gom lúc NÀY có
bao nhiêu khung trong bộ nhớ?

:::opt{correct}
`2` khung — `chiSoKhung(9999)` LÀ `0` còn `chiSoKhung(10000)` LÀ `1`,
hai điểm cách nhau ĐÚNG `1ms` vẫn rơi vào hai khung KHÁC nhau, vì ranh
giới khung LÀ một mốc thời gian cố định, không phải "khoảng cách gần"
:::
:::opt
Vẫn `1` khung — hai điểm ghi cách nhau chỉ `1ms` nên chắc chắn còn quá
GẦN nhau về thời gian để tách RA thành khung riêng
::why
Nhầm "gần nhau về THỜI gian" với "cùng một khung" — nhưng
`chiSoKhung` không hề so sánh khoảng CÁCH giữa hai điểm liên tiếp, nó
chỉ chia `thoiGian` cho `KHUNG_MS` RỒI lấy phần nguyên.

Chỗ lệch: `Math.floor(9999 / 10000)` LÀ `0`, còn `Math.floor(10000 /
10000)` LÀ `1` — dù khoảng cách giữa hai mốc thời gian chỉ LÀ `1ms`,
chúng nằm Ở hai phía CỦA ranh giới `10000`. `ghiDiemTho` gọi
`bo.khung.get(1)`, không tìm thấy, TẠO một khung MỚI — tổng cộng `2`
khung.
::
:::
::::

::::code{#viet_ghi_diem_tho}
Hoàn thiện `ghiDiemTho` — chỉ số khung (`idx`) đã được tính. Còn
thiếu: lấy khung hiện có (hoặc tạo khung rỗng nếu chưa có), cộng dồn
giá trị VÀ tăng số điểm, RỒI ghi khung đó trở lại `Map`.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

const KHUNG_MS = 10000;
interface KhungGom { tong: number; soDiem: number; }
interface BoGomMetric { khung: Map<number, KhungGom>; }
function taoBoGomMetric(): BoGomMetric { return { khung: new Map() }; }
function chiSoKhung(thoiGian: number): number { return Math.floor(thoiGian / KHUNG_MS); }
function ghiDiemTho(bo: BoGomMetric, dh: DongHoMoPhong, giaTri: number): void {
  const idx = chiSoKhung(dh.thoiGianHienTai);
  ___
}
function trungBinhKhung(bo: BoGomMetric, idx: number): number | undefined {
  const k = bo.khung.get(idx);
  if (k === undefined) return undefined;
  return k.tong / k.soDiem;
}

const dhX = taoDongHoMoPhong();
const boX = taoBoGomMetric();
ghiDiemTho(boX, dhX, 5);
ghiDiemTho(boX, dhX, 15);
console.log(trungBinhKhung(boX, 0), boX.khung.size);
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

const KHUNG_MS = 10000;
interface KhungGom { tong: number; soDiem: number; }
interface BoGomMetric { khung: Map<number, KhungGom>; }
function taoBoGomMetric(): BoGomMetric { return { khung: new Map() }; }
function chiSoKhung(thoiGian: number): number { return Math.floor(thoiGian / KHUNG_MS); }
function ghiDiemTho(bo: BoGomMetric, dh: DongHoMoPhong, giaTri: number): void {
  const idx = chiSoKhung(dh.thoiGianHienTai);
  const hienTai = bo.khung.get(idx) ?? { tong: 0, soDiem: 0 };
  hienTai.tong += giaTri;
  hienTai.soDiem += 1;
  bo.khung.set(idx, hienTai);
}
function trungBinhKhung(bo: BoGomMetric, idx: number): number | undefined {
  const k = bo.khung.get(idx);
  if (k === undefined) return undefined;
  return k.tong / k.soDiem;
}

const dhX = taoDongHoMoPhong();
const boX = taoBoGomMetric();
ghiDiemTho(boX, dhX, 5);
ghiDiemTho(boX, dhX, 15);
console.log(trungBinhKhung(boX, 0), boX.khung.size);
```

```typescript title=test
const dhT = taoDongHoMoPhong();
const boT = taoBoGomMetric();
ghiDiemTho(boT, dhT, 10);
ghiDiemTho(boT, dhT, 20);
ghiDiemTho(boT, dhT, 30);
if (trungBinhKhung(boT, 0) !== 20) throw new Error("trung binh khung 0 phai la 20 ((10+20+30)/3)");
const soKhungSauBaDiem = boT.khung.size;
if (soKhungSauBaDiem !== 1) throw new Error("chi 1 khung duoc dung du co 3 diem tho");

tienThoiGian(dhT, KHUNG_MS);
ghiDiemTho(boT, dhT, 100);
const soKhungSauKhiSangKhungMoi = boT.khung.size;
if (soKhungSauKhiSangKhungMoi !== 2) throw new Error("sang khung moi phai tao THEM mot khung, tong 2");
if (trungBinhKhung(boT, 0) !== 20) throw new Error("khung 0 khong duoc thay doi boi diem ghi o khung 1");
if (trungBinhKhung(boT, 1) !== 100) throw new Error("trung binh khung 1 phai la 100");

if (trungBinhKhung(boT, 99) !== undefined) throw new Error("khung chua co diem nao phai tra ve undefined");

const dhB = taoDongHoMoPhong();
const boB = taoBoGomMetric();
tienThoiGian(dhB, 9999);
ghiDiemTho(boB, dhB, 1);
tienThoiGian(dhB, 1);
ghiDiemTho(boB, dhB, 2);
if (boB.khung.size !== 2) throw new Error("t=9999 va t=10000 phai roi vao HAI khung khac nhau");
```

:::hints
- kind: attention
  body: "Con thieu bon buoc: lay khung hien co bang bo.khung.get(idx), neu chua co thi dung { tong: 0, soDiem: 0 }; cong don giaTri vao tong; tang soDiem; ghi khung do tro lai bo.khung.set(idx, ...)."
- kind: strategy
  body: "const hienTai = bo.khung.get(idx) ?? { tong: 0, soDiem: 0 }; hienTai.tong += giaTri; hienTai.soDiem += 1; bo.khung.set(idx, hienTai);"
- kind: one-line
  body: "const hienTai = bo.khung.get(idx) ?? { tong: 0, soDiem: 0 }; hienTai.tong += giaTri; hienTai.soDiem += 1; bo.khung.set(idx, hienTai);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "10 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Điểm thô giờ được nén gọn theo khung. Nhưng một con số trung bình
VƯỢT ngưỡng một LẦN chưa nói lên gì cả — cảnh báo cần thận trọng hơn
thế.
::::

::::reflect{#nghi-lai}
`ghiDiemTho` đánh đổi ĐỘ chi tiết lấy bộ nhớ: một khi điểm thô đã bị
gộp VÀO `tong`/`soDiem`, không còn cách nào lấy LẠI giá trị RIÊNG từng
điểm — chỉ còn trung bình cộng. Đây LÀ đánh đổi có chủ đích của mọi hệ
thống giám sát quy mô lớn, không phải một thiếu sót.
::::

::::checkpoint{mastery=0.70}
::::
